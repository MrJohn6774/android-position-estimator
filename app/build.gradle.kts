import groovy.lang.GroovyObject

plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
    id("org.mozilla.rust-android-gradle.rust-android")
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

    externalNativeBuild {
        ndkBuild.path("jni/Android.mk")
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

extensions.configure<Any>("cargo") {
    val ext = this as GroovyObject
    val libname = "android_position_estimator"

    ext.setProperty("module", "../.")
    ext.setProperty("targets", listOf("arm64"))
    ext.setProperty("libname", libname)
    ext.invokeMethod(
        "exec",
        KotlinClosure2<ExecSpec, Any, Unit>({ spec, _ ->
            spec.environment(
                "RUST_ANDROID_GRADLE_CC_LINK_ARG",
                "-Wl,-z,max-page-size=16384,-soname,lib${libname}.so"
            )
            Unit
        }, this, this)
    )
}

tasks.whenTaskAdded {
    if (name == "javaPreCompileDebug" || name == "javaPreCompileRelease") {
        dependsOn("cargoBuild")
    }
}

tasks.whenObjectAdded {
    if ((this.name == "mergeDebugJniLibFolders" || this.name == "mergeReleaseJniLibFolders")) {
        this.dependsOn("cargoBuild")
        // fix mergeDebugJniLibFolders  UP-TO-DATE
        this.inputs.dir(layout.buildDirectory.dir("rustJniLibs/android"))
    }
}

dependencies {
    implementation("androidx.core:core-ktx:1.9.0")
    implementation("androidx.lifecycle:lifecycle-runtime-ktx:2.6.1")
    implementation("androidx.activity:activity-compose:1.7.2")
}
