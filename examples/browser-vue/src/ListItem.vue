<template>
    <li class="list-group-item">
        <span
            v-for="(chunk, i) in hit.chunks"
            v-bind:key="i"
            v-bind:class="{'font-weight-bold': chunk.highlight}"
        >{{chunk.text}}</span>
    </li>
</template>


<script>
export default {
    name: "list-item",
    props: ["hit"],
}
</script>
