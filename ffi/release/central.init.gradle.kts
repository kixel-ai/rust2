// Sends every Gradle build of a job to Google's mirror of Maven Central: repo.maven.apache.org,
// where the plugin portal also redirects Central's artifacts, blocks some runners' IPs for
// hours, answering 404 or 429 with a retry-after no retry outlasts. A job installs it in
// Gradle's init.d, so it reaches the builds a consumer's app includes, such as Flutter's own.
val mirror = uri("https://maven-central.storage-download.googleapis.com/maven2/")

fun RepositoryHandler.mirrorCentral() = withType(MavenArtifactRepository::class.java).configureEach {
    if (url.host == "repo.maven.apache.org" || url.host == "repo1.maven.org") url = mirror
}

beforeSettings {
    val plugins = pluginManagement.repositories
    plugins.mirrorCentral()
    // The mirror goes ahead of the portal so the portal never redirects to Central. The portal
    // stands in for Gradle's default, which an empty list means, until the build names its own.
    plugins.maven(mirror)
    val portal = plugins.gradlePluginPortal()
    plugins.whenObjectAdded { if (this !== portal) plugins.remove(portal) }
    dependencyResolutionManagement.repositories.mirrorCentral()
}

allprojects {
    buildscript.repositories.mirrorCentral()
    repositories.mirrorCentral()
}
