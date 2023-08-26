<script setup lang="ts">

function menuShow() {
    let barMenu = ref(null).value

    console.log(barMenu)
}

function menuHide() {

}
</script>

<!-- TODO hover tooltips -->

<template>
    <div class="nav">
        <a class="iconsec active" @click="menuShow"><i class="bi bi-list"></i></a>
        <div class="menu" ref="barMenu" style="visibility: hidden">
            <a class="menubtn">Profile</a>
            <a class="menubtn">Settings</a>
            <a class="menubtn">Log out <i class="bi bi-box-arrow-right"></i></a>
        </div>
        <hr>
        <a class="iconsec active">
            <i class="bi bi-house"></i>
        </a>
        <a class="iconsec">
            <i class="bi bi-people-fill"></i>
            <p class="notifcount">1</p>
        </a>
        <a class="iconsec">
            <i class="bi bi-bell-fill"></i>
            <p class="notifcount">9+</p>
        </a>
        <a class="iconsec">
            <i class="bi bi-plus"></i>
        </a>
        <hr>
        <!--pinned DMs-->
        <div class="icongroup">
            <a class="iconsec">
                <i class="bi bi-chat-text-fill"></i>
            </a>
            <div class="content">
                <a class="groupicon">
                    <img src="https://placehold.co/65x65"/>
                    <p class="notifcount">3</p>
                </a>
            </div>
            <a class="groupchevron">
                <i class="bi bi-chevron-up"></i>
            </a>
        </div>
        <hr>
        <!--Groups-->
        <a class="groupicon">
            <img src="https://placehold.co/65x65"/>
            <p class="notifcount" style="visibility: visible">1</p>
        </a>
        <div class="icongroup">
            <a class="iconsec">
                <i class="bi bi-folder2"></i>
            </a>
            <div class="content">
                <a class="groupicon">
                    <img src="https://placehold.co/65x65"/>
                    <!-- <p class="notifcount">3</p> -->
                </a>
            </div>
            <a class="groupchevron">
                <i class="bi bi-chevron-up"></i>
            </a>
        </div>
        <a class="groupicon">
            <img src="https://placehold.co/65x65"/>
            <p class="notifcount">7</p>
        </a>
        <a class="groupicon">
            <img src="https://placehold.co/65x65"/>
            <p class="notifcount">9+</p>
        </a>
    </div>
</template>

<style scoped>
@import url('~/assets/vars.css');

div.nav {
    display: flex;
    flex-direction: column;
    gap: 5px;
    background-color: #0004;
    padding: 5px;
    align-items: center;
    overflow-y: scroll;
}

div.icongroup {
    display: flex;
    background-color: #aaa6;
    flex-direction: column;
    gap: 5px;
    align-items: center;
    border-radius: 10px;
}

div.content {
    display: flex;
    flex-direction: column;
    gap: 5px;
    align-items: center;
}

div.menu {
    padding: 10px;
    font-size: 14pt;
    background-color: #111;
    position: absolute;
    top: 20px;
    left: 95px;
    border-radius: 5px;
    width: 250px;
}

a.menubtn {
    display: flex;
    padding: 8px;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
    background-color: #0000;
    border-radius: 5px;
}

a.menubtn i {
    float: right;
    font-size: 16pt;
}

a.menubtn:hover {
    background-color: #c906;
}

hr {
    color: #999;
    width: 65px;
}

a {
    text-decoration: none;
    display: block;
    padding: 10px;
    transition-property: background-color, color;
    transition-duration: 100ms;
}

a.iconsec {
    font-size: 30pt;
    background-color: #0004;
    border-radius: 10px;
    width: 45px;
    height: 45px;
    text-align: center;
}

a.iconsec.active {
    background-color: #cc08;
    color: #fff;
}

a.iconsec:hover {
    background-color: #c908;
    color: var(--text-link-hover);
    cursor: pointer;
}

a.groupicon {
    width: 65px;
    height: 65px;
    padding: 0;
}

a.groupicon:hover {
    cursor: pointer;
}

a.groupicon img {
    transition-property: opacity;
    transition-duration: 100ms;
    opacity: 50%;
    border-radius: 10px;
}

a.groupicon:hover img {
    opacity: 100%;
}

p.notifcount {
    opacity: 100%;
    background-color: #b22;
    position: relative;
    left: 25px;
    bottom: 50px;
    font-size: 10pt;
    font-weight: bold;
    width: 20px;
    padding: 2px;
    border-radius: 5px;
}

a.groupicon p.notifcount {
    left: 35px;
    bottom: 63px;
    text-align: center;
}

a.groupchevron {
    padding-top: 5px;
    padding-bottom: 5px;
    background-color: #0004;
    width: 45px;
    height: fit-content;
    border-radius: 10px;
    text-align: center;
    font-size: 14pt;
}

a.groupchevron:hover {
    background-color: #c908;
    color: #cc0;
    cursor: pointer;
}

a.groupicon div.hover {
    position: relative;
    visibility: hidden;
    bottom: 70px;
    left: 80px;
    background-color: #222;
    min-width: 100px;
    font-size: 14pt;
    padding: 5px;
    border-radius: 5px;
    box-shadow: 5px 5px 5px 0px #000;
}

a.groupchevron div.hover {
    position: relative;
    visibility: hidden;
    bottom: 70px;
    left: 80px;
    background-color: #222;
    min-width: 100px;
    font-size: 14pt;
    padding: 5px;
    border-radius: 5px;
    box-shadow: 5px 5px 5px 0px #000;
}

a.groupchevron:hover div.hover {
    visibility: visible;
}

a.groupicon:hover div.hover {
    visibility: visible;
}
</style>