// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: { enabled: true }, //DEVELOPMENT MODE
  css: [
    '~/assets/styles.css' //Main stylesheet
  ],

  //App information, like head element and such
  app: {
    head: {
      link: [
        {
          //Bootstrap icons
          rel: 'stylesheet',
          href: 'https://cdn.jsdelivr.net/npm/bootstrap-icons@1.10.5/font/bootstrap-icons.css'
        }
      ]
    }
  },
  runtimeConfig: {
    public: {
      firebaseKey: process.env.FIREBASE_WEB
    }
  }
})
