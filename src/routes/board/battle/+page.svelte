<script>
    // @ts-nocheck 
    import Hexagon from "$lib/Hexagon.svelte";
    import HexagonIndent from "$lib/HexagonIndent.svelte";
    import { invoke } from "@tauri-apps/api/tauri"


    let ids = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z']
    let time_unit = 10
    let time_till_draw = 10000

    function create_grid() {
        let grid = new Array(8)
        for(var i = 0; i < grid.length; i++) {
            grid[i] = new Array(8)
        }
        return grid
    }

    function log_base_n(base, number) {
        return Math.floor(Math.log(number) / Math.log(base));
    }
    function generate_id(num_to_id) {
        if(num_to_id == 0) {
            return "A"
        }
        let id_str = ""
        let cap = log_base_n(ids.length, num_to_id)
        for(let i = 0; i < cap + 1; i++) {
            if(num_to_id == 0){
                id_str += "A"
                break
            }
            let log_n = log_base_n(ids.length, num_to_id)
            let divisor = Math.pow(ids.length, log_n)
            let id_index = Math.floor(num_to_id / divisor)
            id_str += ids[id_index]
            num_to_id -= divisor * id_index
        }
        return id_str
    }

    function hex_click(i, j) {
        selected_champ = grid[i][j]
    }
    async function fetch_board() {
        board = await invoke("fetch_board")
        console.log("board ", board)
        for (let champ_index in board.p1_champions) {
            let location = board.p1_champions[champ_index].location
            board.p1_champions[champ_index].team = true
            board.p1_champions[champ_index].placed_id = generate_id(board.p1_champions[champ_index].id)
            grid[location.x][location.y] = board.p1_champions[champ_index]
        }
        for (let champ_index in board.p2_champions) {
            let location = board.p2_champions[champ_index].location
            board.p2_champions[champ_index].team = false
            board.p2_champions[champ_index].placed_id = generate_id(board.p2_champions[champ_index].id)
            grid[location.x][location.y] = board.p2_champions[champ_index]
        }
    }
    let grid = create_grid()
    let board;
    let selected_champ;
    fetch_board()
</script>

<h1>Board</h1>

<div class="row">
    <div class="column1_noselect">
        {#if selected_champ}
            <title>Unit {selected_champ.placed_id}</title>
            <h1>Type: {selected_champ.of_champ_id}</h1>
            <h1>Status Effects: {selected_champ.status_effects}</h1>
            <h1>Location: {selected_champ.location.x} {selected_champ.location.y}</h1>
            <h1>Movement Progress: {selected_champ.movement_progress}</h1>
            <h1>Target: {generate_id(selected_champ.target)}</h1>
        {/if}
    </div>
    <div class="column2">
        {#each grid as row, i}
            {#if i % 2 == 1}
                <HexagonIndent></HexagonIndent>
            {/if}
            {#each row as hex, j}
                <Hexagon champion = {grid[i][j]} on:click = {() => hex_click(i, j)}></Hexagon>
            {/each}
            <div class = "hex-row"></div>
        {/each}
    </div>
    <div class="column3">
        <h1>GAS GAS GASSSSSSSSSSS</h1>
    </div>
</div>

<style>
    .row {
        display: flex;
    }
    
    .column1_noselect {
        flex: 10%;
        background-color: grey;
        user-select: none;
    }

    .column3 {
        flex: 10%;
        background-color: grey;
        display:grid;
    }

    .column2 {
        flex : 80%;
        background-color: aliceblue;
    }
    .hex-row {
        clear: left;
    }
</style>

