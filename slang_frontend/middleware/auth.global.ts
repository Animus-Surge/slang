import * as https from 'node:https'

// const globalConfig = useRuntimeConfig()

export default defineNuxtRouteMiddleware((to, from) => {
    let auth = useCookie('auth')
    let refresh = useCookie('refresh')

    let bypass = true
    if (bypass) return

    let key = process.env.FIREBASE_WEB

    console.log(auth.value)
    console.log(refresh.value)



    //ONLY RUN THIS IF THE PAGE IS NOT THE LOGIN PAGE
    if (to.path != '/login') {
        console.log('Not going to login...')

        //Logged out
        if (!auth.value && !refresh.value) {
            console.log('Not logged in, going to login page')
            return navigateTo('/login')
        }

        //Refresh token present
        if (!auth.value && refresh.value) {
            let token = refresh.value

            let body = {
                grant_type: 'refresh_token',
                refresh_token: token
            }
            let opts = {
                hostname: 'securetoken.googleapis.com',
                method: 'POST',
                path: 'v1/token?key=' + key
            }

            let req = https.request(opts, (res) => {
                console.log(res)

                res.on('data', (d) => {
                    console.log(d)
                })
            })
            req.on('error', (e) => {
                console.error(e);
            })
            req.write(body)
            req.end()
        }

        //User is logged in, we do absolutely nothing
    }
    
})