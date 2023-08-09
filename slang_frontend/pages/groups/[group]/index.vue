<script setup lang="ts">
const route = useRoute();

console.log(route.params.group);

</script>

<template>
    <div>
        <div class="channellist">
            <!--TODO: move this to a component-->
            
        </div>
        <div class="dash">
            <h2>Group Name</h2>
            <div class="dash-grid">
                <div class="section-grid">
                    <div class="card">
                        <h4>Announcements</h4>
                        <hr>
                        <div class="placeholder-announcement">
                            <h4>Admin name</h4>
                            <p>Placeholder announcement... nothing to see here... anyway hi everyone I'm admin name, and I'm one of the admins in group name! Please be sure to take a look at the rules listed below. Enjoy your time here!</p>
                            <!-- <a class="button" href="#">Read more...</a> -->
                        </div>
                    </div>
                    <div class="card">
                        <h4>Group Rules</h4>
                        <hr>
                        <div class="placeholder-container">
                            <ol>
                                <li>No griefing</li>
                                <li>Wipe your shoes on the mat... keeps the bugs out</li>
                                <li>Please don't be homophobic or transphobic, we are a welcoming community here at group name</li>
                            </ol>
                        </div>
                    </div>
                </div>
                <div class="section-grid">
                    <div class="card">
                        <h4>Following</h4>
                        <hr>
                        <div class="placeholder-container">
                        
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
    grid-template-columns: auto auto;
    justify-items: center;
    align-items: start;
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
    width: 400px;
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

div.placeholder-container ol {
    text-align: start;
}

div.placeholder-announcement {
    margin: 5px;
    border: 1px solid var(--theme1-color-secondary);
    padding: 8px;
    border-radius: 5px;
    background-color: #a605;
}

span.fade {
    color: #999;
}
</style>