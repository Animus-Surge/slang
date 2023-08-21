<script setup lang="ts">
//TODO: move this to a separate file for use elsewhere

const rfc = useRuntimeConfig()
const route = useRoute()
let key = rfc.public.firebaseKey



const signupPath = "https://identitytoolkit.googleapis.com/v1/accounts:signUp?key="
const loginPath = "https://identitytoolkit.googleapis.com/v1/accounts:signInWithPassword?key="

var view = 1

//Login form bindings
let loginErrBox = ref(null)!
let loginEmail = ""
let loginEmailInput = ref(null)!
let loginPwd = ""
let loginPwdInput = ref(null)!
let loginErr = false

//Signup form bindings
let signupErrBox = ref(null)!
let signupErrText = ref(null)!
let signupUname = ""
let signupUnameInput = ref(null)!
let signupEmail = ""
let signupEmailInput = ref(null)!
let signupPwd = ""
let signupPwdC = ""
let signupPwdInput = ref(null)!
let signupPwdCheck = ref(null)!
let signupErr = false

async function loginSubmit() {
	

	
}

async function signupSubmit() {
	//Get ref values
	let unameInput: HTMLElement = signupUnameInput.value!
	let emailInput: HTMLElement = signupEmailInput.value!
	let pwdInput: HTMLElement = signupPwdInput.value!
	let pwdcInput: HTMLElement = signupPwdCheck.value!

	let errBox: HTMLElement = signupErrBox.value!
	let errText: HTMLElement = signupErrText.value!
	
	//Reset global flags
	signupErr = false
	errText.innerText = ""
	errBox.style.visibility = "hidden"

	//Reset field invalid class
	unameInput.classList.remove('invalid')
	emailInput.classList.remove('invalid')
	pwdInput.classList.remove('invalid')
	pwdcInput.classList.remove('invalid')

	// Error checking
	if (
		signupPwd.length === 0 ||
		signupPwdC.length === 0 ||
		signupUname.length === 0 ||
		signupEmail.length === 0
	) {
		errText.innerText = "Cannot leave fields blank"

		if(signupUname.length === 0) unameInput.classList.add('invalid')
		if(signupEmail.length === 0) emailInput.classList.add('invalid')
		if(signupPwd.length === 0) pwdInput.classList.add('invalid')
		if(signupPwdC.length === 0) pwdcInput.classList.add('invalid')

		signupErr = true
	}

	//Check to see if the passwords match
	if (signupPwd != signupPwdC) {
		errText.innerText = "Passwords do not match"

		pwdInput.classList.add('invalid')
		pwdcInput.classList.add('invalid')

		signupErr = true
	}

	//Validate email
	if ( ! (/^(([^<>()\[\]\\.,;:\s@"]+(\.[^<>()\[\]\\.,;:\s@"]+)*)|(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/.test(signupEmail))) {
		errText.innerText = "Email must be valid"

		emailInput.classList.add('invalid')

		signupErr = true
	}

	//If any errors, return
	if (signupErr) {
		errBox.style.visibility = "visible"
		return
	}

	// No errors, let's make the request

	let responsePayload = JSON.stringify({
		email: signupEmail,
		password: signupPwd,
		returnSecureToken: true
	})

	let options = {
		protocol: 'https:',
		port: 443,
		hostname: 'identitytoolkit.googleapis.com',
		path: '/v1/accounts:signUp',
		method: 'POST'
	}

	//Make the request
	let fullUrl = signupPath + key
	let method = "POST"
	let req = new XMLHttpRequest()

	req.onreadystatechange = () => { //TODO: move the httprequest object to global and handle types
		if (req.readyState === XMLHttpRequest.DONE) {
			let status = req.status

			if (status !== 200) {
				//TODO: display an error
				console.log(status)
				console.log(req.statusText)
				console.log(req.response)
			} else {
				let resp = JSON.parse(req.response)

				let auth = useCookie('auth', {maxAge: resp.expiresIn})
				let refresh = useCookie('refresh')

				auth.value = resp.idToken
				refresh.value = resp.refreshToken
				
				//TODO: send the user ID and username over to the database
				return navigateTo('/') //TODO: routeback, return the user to the original page they were
									   // trying to navigate to
			}
		}
	}
	req.open(method, fullUrl, true)
	req.send(responsePayload)

}

//TODO: oauth providers

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
			<div class="err-box">
				<span ref="login-error">
				</span>
			</div>
			<div class="form" ref="login-form">
				<span>Email</span>
				<input required ref="login-email" type="email" v-model="loginEmail">
				<span>Password</span>
				<input required ref="login-pwd" type="password" v-model="loginPwd">
				<button @click="loginSubmit">Log In</button>
			</div>
			<a href="#" @click="swapViews">Or Sign Up instead</a>
		</div>
		<div v-if="view===1" class="card">
			<h2>Sign Up</h2>
			<hr>
			<div ref="signupErrBox" class="err-box" style="visibility: hidden">
				<span ref="signupErrText">
				</span>
			</div>
			<div class="form">
				<span>Username</span>
				<input required ref="signupUnameInput" type="text" v-model="signupUname">
				<span>Email</span>
				<input required ref="signupEmailInput" type="email" v-model="signupEmail">
				<span>Password</span>
				<input required ref="signupPwdInput" type="password" v-model="signupPwd">
				<span>Retype password</span>
				<input required ref="signupPwdCheck" type="password" v-model="signupPwdC">
				&nbsp;
				<button @click="signupSubmit">Sign Up</button>
			</div>
			<a href="#" @click="swapViews">Or Log In instead</a>
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

div.form {
	display: flex;
	flex-direction: column;
	padding: 10px;
	text-align: left;
}

div.form span {
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