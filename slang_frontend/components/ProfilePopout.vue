<script setup lang="ts">
//Toggle logic
import { ref } from 'vue'

const isProfilePopoutOpen = useState('profilePopoutOpen', () => false)
</script>

<!--
    TODO: add button to hide popout, edit profile, etc.
-->

<template>
    <div class="profile-panel" :class="{hidden: !isProfilePopoutOpen}">
        <div class="banner">
            <img src="https://via.placeholder.com/400x200" alt="Banner" />
        </div>
        <div class="panel-content">
            <div class="username-panel">
                <img class="pfp" src="https://via.placeholder.com/64x64" alt="Profile Picture" />
                <div class="panel-text">
                    <h3 id="dispname">Display Name</h3>
                    <h4 id="username">username</h4>
                </div>
            </div>
            <div class="bio-panel">
                <span id="pronouns">Pro/Nouns</span><br>
                <span id="status">Finishing up Slang</span>
                <hr>
                <p>
                    Blah blah fancy bio here yadda yadda yadda some fancy text here<br><br>OwO Whats This???
                </p>
            </div>
        </div>
    </div>
</template>

<style scoped>
div.profile-panel {
    position: absolute;
    top: 0;
    left: 0;
    margin-left: 4rem;

    overflow-x: hidden;
    overflow-y: scroll;

    width: 400px;
    height: 100%;

    background-color: #111;

    box-shadow: 0 0 0.5rem #111;

    transition-property: left;
    transition-duration: 0.5s;
    transition-timing-function: ease-in-out;
}

div.profile-panel.hidden {
    left: -400px;
}

div.panel-content {
    display: flex;
    flex-direction: column;
    align-items: center;

    background-color: #222;

    margin: 8px;
    padding: 8px;

    border-radius: 5px;
}

div.username-panel {
    width: 100%;

    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 5px;

    padding: 8px;
}

div.bio-panel {
    width: 100%;
}

img.pfp {
    border-radius: 50%;
    width: 64px;
    height: 64px;
}

@media screen and (max-width: 768px) {
    div.profile-panel {
        display: none;
    }
}
</style>
