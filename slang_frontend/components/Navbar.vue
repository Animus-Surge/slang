<script setup lang="ts"></script>

<template>
    <div class="navbar">
        <ul>
            <li class="active"><NuxtLink to="/">Dashboard</NuxtLink></li>
            <li><NuxtLink to="/">Notifications</NuxtLink></li>
            <li><NuxtLink to="/">Profile</NuxtLink></li>
        </ul>
    </div>
</template>

<style scoped>
@import url("~/assets/styles.css");

div.navbar ul {
    list-style-type:none;
    margin: 0;
    padding: 10px;
    height: 37px;
    background-color: var(--theme1-color-accent0);
    color: var(--theme1-color-text-light);
}

li a {
    color: var(--theme1-color-text-light);
    text-decoration: none;
    display: block;
    margin-right: 5px;
    margin-left: 5px;
    padding: 8px;
    background-color: var(--theme1-color-accent2);
    float: left;
    border-radius: 5px;
    box-shadow: 4px 4px 0 0 var(--theme1-color-accent1);
    transition-property: box-shadow, background-color, color, translate;
    transition-duration: 100ms;
}

li.active a {
    background-color: var(--theme1-color-secondary);
    color: var(--theme1-color-text-dark);
    box-shadow: 4px 4px 0 0 var(--theme1-color-secondary-dark);
}

li a:hover {
    background-color: var(--theme1-color-text-linkv);
    color: var(--theme1-color-text-dark);
    box-shadow: 2px 2px 0 0 var(--theme1-color-text-linkv-dark);
    translate: 2px 2px;
}

</style>