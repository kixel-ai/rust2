# Install crabx's release binary:
#   powershell -ExecutionPolicy Bypass -c "irm https://crabx.kixel.ai/install.ps1 | iex"
# CRABX_VERSION picks a version instead of the newest stable one on crates.io, and
# CRABX_INSTALL_DIR the directory instead of ~\.local\bin.
# A script block keeps its variables out of the session `iex` runs it in, and errors throw
# rather than exit, which would close that session.
& {
	$ErrorActionPreference = 'Stop'
	$ProgressPreference = 'SilentlyContinue'
	# Windows PowerShell 5.1 may not offer TLS 1.2 by default.
	[Net.ServicePointManager]::SecurityProtocol = [Net.ServicePointManager]::SecurityProtocol -bor [Net.SecurityProtocolType]::Tls12
	$web = New-Object System.Net.WebClient

	# The OS's architecture, not this process's: an x64 PowerShell on Arm reports x64.
	$arch = [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture
	$target = switch ("$arch") {
		'X64' { 'x86_64-pc-windows-msvc' }
		'Arm64' { 'aarch64-pc-windows-msvc' }
		default { throw "crabx install: no prebuilt crabx for '$arch'; install it with ``cargo install crabx``" }
	}

	$version = $env:CRABX_VERSION
	if (-not $version) {
		# The highest version on crates.io that is neither yanked nor a prerelease.
		$index = $web.DownloadString('https://index.crates.io/cr/ab/crabx')
		$version = $index -split "`n" | Where-Object { $_ } | ForEach-Object { $_ | ConvertFrom-Json } |
			Where-Object { -not $_.yanked -and ($_.vers -replace '\+.*', '') -notmatch '-' } |
			Sort-Object { [version]($_.vers -replace '\+.*', '') } | Select-Object -Last 1 -ExpandProperty vers
		if (-not $version) { throw 'crabx install: crates.io has no stable crabx release' }
	}
	if ($version -notmatch '^[0-9A-Za-z.+-]+$') { throw "crabx install: invalid version $version" }

	$dir = if ($env:CRABX_INSTALL_DIR) { $env:CRABX_INSTALL_DIR } else { Join-Path $HOME '.local\bin' }
	$name = "crabx-$target-v$version"
	$url = "https://github.com/kixel-ai/rust2/releases/download/crabx-v$version/$name.tgz"
	$temp = Join-Path ([System.IO.Path]::GetTempPath()) ([System.IO.Path]::GetRandomFileName())
	New-Item -ItemType Directory $temp | Out-Null
	try {
		Write-Host "crabx install: downloading crabx $version for $target"
		try {
			$web.DownloadFile($url, "$temp\crabx.tgz")
		} catch {
			throw "crabx install: cannot download ${url}; crabx $version's binaries may not be released yet: $_"
		}
		# Windows 10 1803 and later ship bsdtar.
		tar -xzf "$temp\crabx.tgz" -C $temp "$name/crabx.exe"
		if ($LASTEXITCODE -ne 0) { throw "crabx install: cannot extract $name.tgz" }
		New-Item -ItemType Directory -Force $dir | Out-Null
		Move-Item -Force "$temp\$name\crabx.exe" (Join-Path $dir 'crabx.exe')
	} finally {
		Remove-Item -Recurse -Force $temp
	}
	Write-Host "crabx install: installed $(Join-Path $dir 'crabx.exe')"

	$path = [Environment]::GetEnvironmentVariable('Path', 'User')
	if (($path -split ';') -notcontains $dir) {
		[Environment]::SetEnvironmentVariable('Path', ((@($path, $dir) | Where-Object { $_ }) -join ';'), 'User')
		$env:Path = "$env:Path;$dir"
		Write-Host "crabx install: added $dir to your PATH; restart other terminals to use it"
	}
}
