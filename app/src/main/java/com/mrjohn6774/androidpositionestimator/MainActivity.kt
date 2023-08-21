package com.mrjohn6774.androidpositionestimator

import android.os.Bundle
import android.app.NativeActivity

class MainActivity : NativeActivity() {
    companion object {
        init {
//            System.loadLibrary("libmain")
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
    }
}