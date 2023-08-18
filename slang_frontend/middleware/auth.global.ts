export default defineNuxtRouteMiddleware((to, from) => {
    const authCookie = useCookie('slang-auth')

    //Let's first check the path we're going to
    if (authCookie && to.path == "/login") {

    }
})