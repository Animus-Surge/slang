<script setup lang="ts">
</script>

<!--App dashboard, redirects to login/signup page if not authenticated. SEE MIDDLEWARE-->

<template>
    <div class="dash">
        <h2>Dashboard</h2>
        <div class="dash-grid">
            <div class="section-grid">
                <div class="card">
                    <h4>Recent News</h4>
                    <hr>
                    <img src="https://placehold.co/280x100"/>
                    <p>Wow! A news entry! I wonder what it reads... if only there was actually some news here, otherwise the read more... button would actually do something instead of being non-functional...</p>
                    <a class="button" href="#">Read more...</a>
                </div>
                <div class="card">
                    <h4>Quick Links</h4>
                    <hr>
                    <div class="placeholder-container">
                        <a class="placeholder-link" href="#">Profile</a>
                        <a class="placeholder-link" href="#">Settings</a>
                    </div>
                </div>
            </div>
            <div class="section-grid">
                <div class="card">
                    <h4>Recent Activity</h4>
                    <hr>
                    <div class="placeholder-container">
                        <div class="placeholder-ping">
                            <img style="border-radius:50%" src="https://placehold.co/50/blue/blue"/>
                            <div class="placeholder-ping-text">
                                <p><strong>Someone</strong></p>
                                <p>Hahaha @you, I completely get where you're coming from now!</p>
                            </div>
                        </div>
                        <div class="placeholder-ping">
                            <img style="border-radius:50%" src="https://placehold.co/50/yellow/yellow"/>
                            <div class="placeholder-ping-text">
                                <p><strong>Someone else</strong></p>
                                <p>@role guys it's time...</p>
                            </div>
                        </div>
                        <div class="placeholder-ping">
                            <img style="border-radius:50%" src="https://placehold.co/50/purple/purple"/>
                            <div class="placeholder-ping-text">
                                <p><strong>Group admin</strong></p>
                                <p>@group Hi everyone, time for the weekly announcement reg<span class="fade">...</span></p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
            <div class="section-grid">
                <div class="card">
                    <h4>Friends</h4>
                    <hr>
                    <div class="placeholder-ping">
                        <img style="border-radius:50%" src="https://placehold.co/50/green/green"/>
                        <div class="placeholder-ping-text">
                            <p><strong>GalaxyFren101</strong></p>
                            <p>0 unread messages</p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
@import url('~/assets/vars.css');

div.dash {
    display: flex;
    flex-direction: column;
    align-items: center;
    width: 100vw;
    height: 80vh;
    margin: 15px;
    padding: 10px;
    border-radius: 8px;
    box-shadow: 5px 5px 3px 3px #0005;
    background-color: #0007;
}

div.dash-grid {
    display: grid;
    grid-template-columns: auto auto auto;
    width: 50vw;
    margin-top: 10px;
}

div.section-grid {
    display: grid;
    grid-template-columns: auto;
    gap: 10px;
}

div.card {
    text-align: center;
    background-color: #a605;
    padding: 5px;
    padding-bottom: 10px;
    border-radius: 5px;
    width: 300px;
}

a.button {
    margin: 5px auto auto auto;
    width: fit-content;
    color: var(--theme1-color-text-light);
    text-decoration: none;
    display: block;
    padding: 8px;
    background-color: var(--theme1-color-accent2);
    border-radius: 5px;
    box-shadow: 4px 4px 0 0 var(--theme1-color-accent1);
    transition-property: box-shadow, background-color, color, translate;
    transition-duration: 100ms;
}

a.button:hover {
    background-color: var(--theme1-color-text-linkv);
    color: var(--theme1-color-text-dark);
    box-shadow: 2px 2px 0 0 var(--theme1-color-text-linkv-dark);
    translate: 2px 2px;
}

div.placeholder-ping {
    display: flex;
    flex-direction: row;
    align-items: center;
    background-color: #222c;
    border-radius: 5px;
    padding: 5px;
    height: 70px;

    transition-property: background-color;
    transition-duration: 100ms;
}

div.placeholder-ping:hover {
    cursor: pointer;
    background-color: #222;
}

div.placeholder-ping-text {
    text-align: left;
    margin-left: 5px;
}

div.placeholder-container {
    display: grid;
    grid-template-columns: auto;
    gap: 8px;
}

span.fade {
    color: #999;
}
</style>