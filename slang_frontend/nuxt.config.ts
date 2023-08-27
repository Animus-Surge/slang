import * as path from 'path'

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: { enabled: true }, //DEVELOPMENT MODE
  css: [
    '~/assets/styles.css', //Main stylesheet
    '~/assets/vars.css',   //Vars stylesheet,
    // 'animate.css',
    '~/assets/bootstrap-icons.min.css',
  ],
  runtimeConfig: {
    public: {
      firebaseKey: process.env.FIREBASE_WEB
    }
  }
})
