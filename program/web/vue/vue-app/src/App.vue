<template>
    <div id="app">
        <!-- <Home msg="Welcome to Your Vue.js App" /> -->
		<Header></Header>
        <router-view />
    </div>
</template>

<script>
import Header from "./components/Header.vue";

export default {
    name: "app",
    components: {
        Header
    }
};
</script>

<style lang="scss">
@import "~bulma/bulma";
</style>
