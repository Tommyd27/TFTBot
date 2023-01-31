<script>
    import { invoke } from "@tauri-apps/api/tauri";


    async function fetch_previous_result() {
        invoke("fetch_outcomes")
    }
</script>


<h1>Previous Results</h1>
<button on:click = {fetch_previous_result}>PRESS ME PRESS ME OM NOM NOM</button>