<script setup lang="ts">
// let dotenv = require('dotenv').config



// let key = process.env.FIREBASE_WEBKEY

const loginPath = "https://identitytoolkit.googleapis.com/v1/accounts?signUp="

// var someToken = useState('auth')

var view = 0

//Login form bindings
let loginErrBox = document.getElementById("login-error")!
let loginForm = document.getElementById("login-form")
let loginEmail = ""
let loginEmailInput = document.getElementById("login-email")!
let loginPwd = ""
let loginPwdInput = document.getElementById("login-pwd")!
let loginErr = false

//Signup form bindings
let signupErrBox = document.getElementById("signup-error")!
let signupForm = document.getElementById("signup-form")
let signupEmail = ""
let signupEmailInput = document.getElementById("signup-email")!
let signupPwd = ""
let signupPwdInput = document.getElementById("signup-pwd")!
let signupErr = false

loginForm?.addEventListener("submit", loginSubmit)
loginForm?.addEventListener("invalid", (evt) => evt.preventDefault())
signupForm?.addEventListener("submit", signupSubmit)
signupForm?.addEventListener("invalid", (evt) => evt.preventDefault())

function loginSubmit(evt: SubmitEvent) {
    evt.preventDefault()

    if (loginEmail.length === 0 || loginPwd.length === 0) {
        loginErr = true
        loginErrBox.innerText = "Email or password cannot be blank"

        if (loginEmail.length === 0) loginEmailInput.classList.add('invalid')
        if (loginPwd.length === 0) loginPwdInput.classList.add('invalid')
    }

    let responsePayload = {
        email: loginEmail,
        password: loginPwd
    }
}

function signupSubmit(evt: SubmitEvent) {
    evt.preventDefault()
}

function swapViews() {
    if (view === 0) view = 1;
    else if (view === 1) view = 0; 
}

</script>

<template>
    <div class="login-root">
        <div v-if="view===0" class="card">
            <h2>Sign In</h2>
            <hr>
            <div class="err-box" v-if="false">
                <span id="login-error">

                </span>
            </div>
            <form id="login-form">
                <span>Email</span>
                <input id="login-email" type="email" v-model="loginEmail">
                <span>Password</span>
                <input id="login-pwd" type="password" v-model="loginPwd">
                <button type="submit">Log In</button>
            </form>
            <a href="#" @click="swapViews">Or Sign Up instead</a>
        </div>
    </div>
</template>

<style scoped>

div.login-root {
    margin-left: auto;
    margin-right: auto;
    margin-top: 50vh;
    translate: 0 -50%;
}

div.card {
    text-align: center;
    background-color: #0003;
    padding: 10px;
    border-radius: 5px;
    max-width: 300px;
}

div.err-box {
    background-color: #5005;
    padding: 5px;
    border: 1px solid #a00;
    border-radius: 5px;
    width: fit-content;
    margin-left: auto;
    margin-right: auto;
}

form {
    display: flex;
    flex-direction: column;
    padding: 10px;
    text-align: left;
}

form span {
    color: #ccc;
}

input {
    width: 250px;
    margin-top: 8px;
    margin-bottom: 5px;
    font-size: 14pt;
    padding: 5px;
    border-radius: 5px;
    border: unset;
    color: #ccc;
    background-color: #1117;
    transition-property: background-color, color;
    transition-duration: 100ms;
}

input:hover {
    background-color: #5556;
    color: #fff;
}

input:invalid, input.invalid {
    background-color: #200;
}

input:focus {
    background-color: #8728;
}

button {
    background-color: #c90;
    border: unset;
    font-size: 14pt;
    padding: 5px;
    border-radius: 5px;
    transition-property: background-color;
    transition-duration: 100ms;
}

button:hover {
    background-color: #cc0;
}

</style>