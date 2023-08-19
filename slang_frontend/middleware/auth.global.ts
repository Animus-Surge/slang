export default defineNuxtRouteMiddleware((to, from) => {
    const authState = useState("auth")

    if(!authState && to.path != '/login') {
        navigateTo('login')
    }
})