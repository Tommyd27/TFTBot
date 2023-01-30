<script>
    import { invoke } from "@tauri-apps/api/tauri";


    async function fetch_board() {
        let board = await invoke("fetch_board")
        console.log(board)
    }
</script>



<h1>HELLO</h1>
<button on:click = {fetch_board}>PRESS</button>