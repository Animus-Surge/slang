<script setup lang="ts">
</script>

<!--App dashboard, redirects to login/signup page if not authenticated. SEE MIDDLEWARE-->

<template>
    <div class="template-root">
        <div class="content-group-v">
            <div class="panel">
                <h3>News</h3>
                <p>
                    Hi! News panel here with the latest update on the inner workings of Slang! Meet
                    your new system for showing the latest updates and news shared by the devs, as
                    well as the latest activity in groups and from your friends! Quick links are
                    here too for quick access to your favourite groups and more!
                </p>
                <br>
                <a href="#">Read More</a>
            </div>
            <div class="panel">
                <h3>Quick Links</h3>
                <br>
                <div class="inset">
                    <a class="button" href="#">Favourite Group 1</a>
                    <a class="button" href="#">Favourite Group 2</a>
                </div>
            </div>
        </div>
        <div class="content-group-v">
            <div class="panel">
                <h3>Recent Activity</h3>
                <br>
                <div class="group">
                    <div class="horiz-item">
                        <img class="icon" src="https://placehold.co/45x45/red/red"/>
                        <div class="text">
                            <h4>Username &rarr; Group</h4>
                            <p>Hewwo @you! Hru</p>
                        </div>
                    </div>
                </div>
            </div>
            <div class="panel">
                <h3>Discover</h3>
                <br>
                <div class="group">
                    <div class="item">
                        <h4>Something</h4>
                        <br>
                        <p>lorem aef aufhlabhe ifuaeb uwe inao wnbuif iwabi a uabe osuluw naewuf</p>
                        <br>
                        <a href="#">Read More</a>
                        <img src="https://placehold.co/350x200"/>
                    </div>
                </div>
            </div>
        </div>
        <div class="content-group-v">
            <div class="panel">
                <h3>Friends</h3>
                <br>
                <div class="group">
                    <div class="horiz-item">
                        <img class="icon" src="https://placehold.co/45x45/red/red"/>
                        <div class="text">
                            <h4>Friend name</h4>
                            <p>No unread messages</p>
                        </div>
                    </div>
                    <div class="horiz-item">
                        <img class="icon" src="https://placehold.co/45x45/red/red"/>
                        <div class="text">
                            <h4>Friend name</h4>
                            <p>No unread messages</p>
                        </div>
                    </div>
                    <div class="horiz-item">
                        <img class="icon" src="https://placehold.co/45x45/red/red"/>
                        <div class="text">
                            <h4>Friend name</h4>
                            <p>No unread messages</p>
                        </div>
                    </div>
                </div>
            </div>
            <div class="panel">
                <h3></h3>
            </div>
        </div>
    </div>
</template>

<style scoped>
@import url('~/assets/vars.css');

img {
    border-radius: 5px;
}

div.template-root {
    padding: 8px;
    display: flex;
    margin: auto;
    flex-direction: row;
    gap: 8px;
}

div.content-group-v {
    width: 375px;
    display: flex;
    flex-direction: column;
    gap: 8px;
}

div.panel {
    background-color: #c808;
    padding: 5px;
    border-radius: 5px;
}

div.panel h3 {
    text-align: center;
}

/* Activity Panel */
div.horiz-item {
    background-color: #2224;
    padding: 3px;
    border-radius: 3px;
    display: flex;
    flex-direction: row;
    gap: 3px;
    transition-property: background-color;
    transition-duration: 100ms;
    cursor: pointer;
}

div.horiz-item:hover {
    background-color: #2228;
}

img.icon {
    border-radius: 50%;
}

div.inset {
    display: flex;
    flex-direction: column;
    gap: 3px;
}

a.button {
    display: block;
    padding: 8px;
    text-align: center;
    background-color: #888;
    color: #000;
    transition-property: background-color;
    transition-duration: 100ms;
    border-radius: 3px;
}

a.button:hover {
    background-color: #880;
    text-decoration: none;
}

div.group {
    display: flex;
    flex-direction: column;
    gap: 5px;
}
</style>