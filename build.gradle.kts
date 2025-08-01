buildscript {
    repositories {
        google()
        maven(
            "https://plugins.gradle.org/m2/"
        )
    }
}

// Top-level build file where you can add configuration options common to all sub-projects/modules.
plugins {
    id("com.android.application") version "8.11.1" apply false
    id("org.jetbrains.kotlin.android") version "1.8.10" apply false
    id("org.mozilla.rust-android-gradle.rust-android") version "0.9.6" apply false
}
