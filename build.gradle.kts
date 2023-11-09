// Top-level build file where you can add configuration options common to all sub-projects/modules.
plugins {
    id("com.android.application") version "8.1.3" apply false
    id("org.jetbrains.kotlin.android") version "1.8.10" apply false
    id("com.github.willir.rust.cargo-ndk-android") version "0.3.4" apply false
}

tasks.register<Delete>("clean") {
    delete(rootProject.buildDir)
    delete("$projectDir/app/src/main/jniLibs")
}
