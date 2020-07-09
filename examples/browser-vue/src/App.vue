<template>
    <div>
        <h1>LucidSuggest demo</h1>
        <br>
        <form>
            <div class="input-group mb-3">
                <input
                    type="text"
                    id="search-input"
                    class="form-control"
                    v-model="input"
                    @input="search"
                >
                <div class="input-group-append">
                    <span class="input-group-text" id="basic-addon2">
                        <icon-loupe />
                    </span>
                </div>
            </div>
        </form>
        <ul class="list-group" id="search-results">
            <list-item
                v-for="hit in hits"
                v-bind:key="hit.record.id"
                v-bind:hit="hit"
            />
        </ul>
    </div>
</template>


<script>
import IconLoupe from "./IconLoupe"
import ListItem from "./ListItem"
import {LucidSuggest} from 'lucid-suggest/en'
import DATA from './e_commerce.json'

const suggest = new LucidSuggest()
suggest.setRecords(DATA)

export default {
    name: "App",
    data: () => ({
        input: "",
        hits: [],
    }),
    methods: {
        search: function() {
            suggest.search(this.input).then(hits => {
                this.hits = hits
            })
        },
    },
    components: {
        IconLoupe,
        ListItem,
    }
}
</script>
