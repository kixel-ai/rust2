package dev.rust2.consumer

import android.os.Bundle
import com.facebook.react.ReactActivity
import com.facebook.react.ReactActivityDelegate
import com.facebook.react.defaults.DefaultNewArchitectureEntryPoint.fabricEnabled
import com.facebook.react.defaults.DefaultReactActivityDelegate

class MainActivity : ReactActivity() {
  override fun getMainComponentName(): String = "Consumer"

  // The case to run, from the intent's `case` extra, as the component's `case` prop.
  override fun createReactActivityDelegate(): ReactActivityDelegate =
      object : DefaultReactActivityDelegate(this, mainComponentName, fabricEnabled) {
        override fun getLaunchOptions(): Bundle =
            Bundle().apply { putString("case", intent.getStringExtra("case")) }
      }
}
