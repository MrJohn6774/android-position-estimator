import com.android.build.gradle.internal.tasks.factory.dependsOn

plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
    id("com.github.willir.rust.cargo-ndk-android")
}

android {
    namespace = "com.mrjohn6774.androidpositionestimator"
    compileSdk = 33
    ndkVersion = "25.2.9519653"

    defaultConfig {
        applicationId = "com.mrjohn6774.androidpositionestimator"
        minSdk = 24
        targetSdk = 33
        versionCode = 1
        versionName = "1.0"

        // Reference: https://github.com/MatrixDev/GradleAndroidRustPlugin/issues/3#issuecomment-1505416835
        ndk {
            abiFilters.add("arm64-v8a")
        }
        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
        vectorDrawables {
            useSupportLibrary = true
        }
    }

    buildTypes {
        release {
            isMinifyEnabled = false
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }
        getByName("debug") {
            isJniDebuggable = true
        }
    }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_1_8
        targetCompatibility = JavaVersion.VERSION_1_8
    }
    kotlinOptions {
        jvmTarget = "1.8"
    }
    buildFeatures {
        compose = true
    }
    composeOptions {
        kotlinCompilerExtensionVersion = "1.4.3"
    }
    packaging {
        resources {
            excludes += "/META-INF/{AL2.0,LGPL2.1}"
        }
    }
}

cargoNdk {
    // Reference: https://github.com/willir/cargo-ndk-android-gradle
    module = "rust"
    targets = arrayListOf("arm64")
}

tasks.register<Exec>("cargoClean") {
    executable("cargo")     // cargo.cargoCommand
    args("clean")
    workingDir("$projectDir/../${cargoNdk.module}")
}
tasks.clean.dependsOn("cargoClean")

dependencies {

    implementation("androidx.core:core-ktx:1.9.0")
    implementation("androidx.lifecycle:lifecycle-runtime-ktx:2.6.1")
    implementation("androidx.activity:activity-compose:1.7.2")
}
