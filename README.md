# Your Project Name Here

This template gives you only what you need to get up and running with a new project that uses electron and holochain!

Check out the video walkthrough, just note that some minor details may have changed since then, but it will still be super useful to understand how to utilize this template: https://www.youtube.com/watch?v=jFraPKl2rPk.

[IMPORTANT! Check Dependency Versions Information (Holochain etc)](#dependency-versions-information)

__Table of Contents__
- [Set Up after Clone](#set-up-after-clone)
  - [App Icon Images](#app-icon-images)
- [Run Locally and Develop on your Computer](#run-locally-and-develop-on-your-computer)
- [Multi User Development Testing](#multi-user-development-testing)
- [Building / Packaging](#building--packaging)
- [Versioning for User Data](#versioning-for-user-data)
- [IMPORTANT! Dependency Versions Information (Holochain etc)](#dependency-versions-information)

## Set Up after Clone

Global find and replace:

`ElectronHolochainTemplate`: replace with the actual name you wish to see appear in users desktop launcher icons: e.g. "Acorn"

`com.some-domain-name.app-name`: replace with an Apple ["bundle Id"](https://developer.apple.com/documentation/appstoreconnectapi/bundle_ids) that is registered on your Apple Developer account

### App Icon Images

Replace `electron/build/icon.icns`. This one is utilized by MacOS.

Replace `electron/build/icon.ico`. This one is utilized by Windows

[TODO: linux](https://github.com/lightningrodlabs/electron-holochain-template/issues/2)

## Run Locally and Develop on your Computer

_Prerequisites_

- Have rust language (stable) installed on your system
- Have nodejs version 16 installed on your system

Then run

- `npm run install-deps`
- `npm run dev`

In the future, just run `npm run dev` anytime to develop.

When you run `npm run dev` a `user-data/` directory is created and this is where user data including private keys, and also data generated through use of the app is stored.

You can run `npm run user-data-reset` if you have user data in development, but you want to clear it, and start over with fresh identities.

> NOTE: if you see a blank screen once electron launches the app, refresh the page (using View -> Reload or Cmd/Ctrl-R) to see app contents.

### Commands that are more specific to your use case:

**dna**

- Have rust language (stable) installed on your system, then...
- `npm run happ-install`: installs wasm32 compilation target for rust as well as the Holochain CLI
- `npm run happ-pack`: compiles zomes into wasm and packages them into a .dna and a .happ using Holochain CLI 
- `npm run happ-reset`: runs `happ-pack` and clears user data (Run this anytime you change the code in `happ` folder during development)

**web** (user interface)

- Use nodejs version 16
- `npm run web-install`
- `npm run web`

**electron**

- `npm run electron-install`
- `npm run electron-tsc` (**needs to be re-run whenever electron folder source code changes**)
- `npm run electron`

## Multi-User Development Testing
Some features to develop and test require running two instances of the app simultaneously. The project is set up with that in mind.

run the following commands in separate terminal instances (must have a running instance of acorn for the first user, either by running `npm run dev` or the below commands without the `2`):

- `npm run web2`
- `npm run electron2`

After running these commands, a `user2-data/` directory is created with user data. It too can be cleared by running `npm run user-data-reset`.

## Building / Packaging

To build:

- `npm run build`

The packaged executables can be found in `electron/out`.

In order to get cross-platform builds, just tag your repository like `v0.0.1` and push those tags to Github. CI will automatically start running a build, under the "Release" action.

> Macos: You will need to have set the following environment variables as repository secrets:
> - APPLE_CERTIFICATE
> - APPLE_CERTIFICATE_PASSWORD
> - APPLE_DEV_IDENTITY
> - APPLE_ID
> - APPLE_PASSWORD
> 
> See: https://hackmd.io/@connoropolous/HkxeYYgzo
>
> There is a sixth environment variable which is useful to set, like this: `DEBUG: electron-osx-sign*,electron-notarize*`. This allows for useful logging outputs from the signing and notarizing process. This env var is set automatically when running on CI, in the "Release" Github Action.


## Versioning For User Data

Each version of the app will either change, or not change, the paths to the user data folders in use by the application. 

The user data will be located under a folder with the same name as the value given under the [`name` property of the file `electron/package.json`](./electron/package.json#L2) in the platform specific appData folder, as specified by `appData` here: https://www.electronjs.org/docs/latest/api/app#appgetpathname

It is then in a specific sub-folder that relates to one of two types of data: 
- source chain and DHT -> `databases-${DATABASES_VERSION_ID}`
- private keys -> `keystore-${KEYSTORE_VERSION_ID}`

DATABASES_VERSION_ID and KEYSTORE_VERSION_ID are defined in `electron/src/holochain.ts` and can be modified as needed in order to jump to new versions of holochain, or a new app DNA.

You can tweak DATABASES_VERSION_ID and KEYSTORE_VERSION_ID independently. 

DATABASES_VERSION_ID should be incremented when a new DNA is in use. It will cause users to have to re-create profiles and re-instate data they've previously added.

KEYSTORE_VERSION_ID should be incremented if you want users to have to switch and generate new keys for any reason.


## Dependency Versions Information

This project is currently using:

holochain-runner [v0.5.2](https://github.com/Sprillow/holochain-runner/releases/tag/v0.5.2)

which has an underlying `holochain` version of [0.1.0-beta-rc.2](https://github.com/holochain/holochain/releases/tag/holochain-0.1.0-beta-rc.2)

expects an HAPP built with
- HDK [v0.1.0-beta-rc.1](https://docs.rs/hdk/0.1.0-beta-rc.1/hdk/index.html)
- HDI [v0.2.0-beta-rc.1](https://docs.rs/hdi/0.2.0-beta-rc.1/hdi/index.html)

and [electron 22](https://www.electronjs.org/docs/latest/api/app)




