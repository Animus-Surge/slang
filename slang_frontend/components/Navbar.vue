<script setup lang="ts"></script>

<template>
    <div class="navbar">
        <ul>
            <li class="active"><NuxtLink to="/">Dashboard</NuxtLink></li>
            <li><NuxtLink to="/">Notifications</NuxtLink></li>
            <li><NuxtLink to="/">Profile</NuxtLink></li>
        </ul>
        <!--TODO: server list-->
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
    display: block;
    float: left;
    padding: 8px;
    margin-right: 10px;
    color: var(--theme1-color-text-light);
    text-decoration: none;
    background-color: var(--theme1-color-accent1);

    /* Transitions */
    transition-property: color, background-color;
    transition-duration: 100ms;
}

li.active a {
    color: var(--theme1-color-text-dark);
    background-color: var(--theme1-color-secondary);
}

li a:hover {
    color: var(--theme1-color-text-dark);
    background-color: var(--theme1-color-tertary);
    text-decoration: none;
}

</style>