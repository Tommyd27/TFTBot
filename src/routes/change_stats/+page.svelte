<script>
    import { invoke } from "@tauri-apps/api/tauri"

    async function fetch_champs() {
        console.log(await invoke("retrieve_all_unit_ids"))
        return await invoke("retrieve_all_unit_ids")
    }
    async function fetch_items() {
        return await invoke("retrieve_all_item_ids")
    }

    let champs = fetch_champs()
    let items = fetch_items()
</script>

{#await champs}
    <p>...fetching champs</p>
{:then}
    <p></p>
{:catch error}
    <p>Failure to fetch champs {error}</p>
{/await}

{#await items}
    <p>...fetching items</p>
{:then}

{:catch error}
    <p>Failure to fetch items {error}</p>
{/await}
<h1>Change Stats</h1>
