<script>
    // @ts-nocheck 
    import Hexagon from "$lib/Hexagon.svelte";
    import HexagonIndent from "$lib/HexagonIndent.svelte";
    import { invoke } from "@tauri-apps/api/tauri"

    let play = false;

    let ids = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z']
    let time_unit = 10
    let time_till_draw = 10000

    function create_grid() {
        grid = new Array(8)
        for(var i = 0; i < grid.length; i++) {
            grid[i] = new Array(8)
        }
    }

    function log_base_n(base, number) {
        return Math.floor(Math.log(number) / Math.log(base));
    }
    function generate_id(num_to_id) {
        if(num_to_id == 0) {
            return "A"
        }
        if(num_to_id == 255) {
            return "None"
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
        create_grid()
        board = await invoke("fetch_board")
        if (board.p1_champions.length == 0 || board.p2_champions.length == 0) {
            console.log("ovaaaa")
            show_over = true
            await new Promise(r => setTimeout(r, 2000));
            show_over = false
        }
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
    function pause() {
        play = false;
    }
    async function play_at() {
        if (play) { return }
        play = true
        while (play) {
            let start = Date.now()
            await invoke("simulate_x_ticks", {numTicks : play_at_ticks})
            await fetch_board()
            let time_taken = Date.now() - start
            if (time_taken < 1000) {
                await new Promise(r => setTimeout(r, 1000 - time_taken));
            }
        }
        
    }
    async function jump_forward() {
        await invoke("simulate_x_ticks", {numTicks : jump_ticks_num})
        await fetch_board()
    }
    async function save_battle() {
        if (board.p1_champions.length > 0 && board.p2_champions.length > 0 ) {
            show_battle_over = 100
            await new Promise(r => setTimeout(r, 500));
            show_battle_over = 0
            return
        }

    }
    let grid;
    let play_at_ticks = 5;
    let jump_ticks_num = 100;
    let board;
    let selected_champ;
    let show_battle_over = 0
    let show_over = false;
    fetch_board()
</script>

<div class="row">
    <div class="column1_noselect">
        {#if selected_champ}
            <title>Unit {selected_champ.placed_id}</title>
            <h1>Type: {selected_champ.of_champ_id}</h1>
            <h1>Location: {selected_champ.location.x} {selected_champ.location.y}</h1>
            <h1>Movement Progress: {selected_champ.movement_progress}</h1>
            <h1>Target: {generate_id(selected_champ.target)}</h1>
            <h1>Status Effects: {selected_champ.se.length}</h1>
            <h1>AP: {selected_champ.ap.toFixed(2)}</h1>
            <h1>AD: {selected_champ.ad.toFixed(2)}</h1>
            <h1>AR: {selected_champ.ar.toFixed(2)}</h1>
            <h1>Attack Speed: {selected_champ.attack_speed.toFixed(2)}</h1>
            <h1>Attack Speed Mod: {selected_champ.attack_speed_modifier.toFixed(2)}</h1>
            <h1>Auto Attack Delay: {selected_champ.auto_attack_delay}</h1>
            <h1>Banished?: {selected_champ.banish}</h1>
            <h1>Current Mana: {selected_champ.cm}</h1>
            <h1>Crit Rate: {selected_champ.cr}</h1>
            <h1>Crit Damage: {selected_champ.crit_damage.toFixed(2)}</h1>
            <h1>Dodge Chance: {selected_champ.dc}</h1>
            <h1>Inc Damage Modifier: {selected_champ.incoming_damage_modifier.toFixed(2)}</h1>
            <h1>HP: {selected_champ.health.toFixed(2)}</h1>
            <h1>Initial HP: {selected_champ.initial_hp.toFixed(2)}</h1>
            <h1>Items: {selected_champ.items}</h1>
            <h1>Mana Cost: {selected_champ.mc}</h1>
            <h1>Shed: {selected_champ.shed}</h1>
            <h1>Shields: {selected_champ.shields.length}</h1>
            <h1>Target Cells: {selected_champ.target_cells.x} {selected_champ.target_cells.y}</h1>
            <h1>Target Cooldown: {selected_champ.target_cooldown}</h1>
            <h1>Targetable: {selected_champ.targetable}</h1>
            <h1>Titan's Resolve Stacks: {selected_champ.titans_resolve_stacks}</h1>
            <h1>Zap: {selected_champ.zap}</h1>

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
        <button on:click = {play_at}>Play</button>
        <button on:click = {pause}>Pause</button>
        <button on:click = {jump_forward}>Jump Forward</button>
        <label>Play at</label>
        <input type = "number" min = 1 max = 20 bind:value = {play_at_ticks}>
        <label>Jump Forward</label>
        <input type = "number" min = 1 max = 20000 bind:value = {jump_ticks_num}>
        <button on:click = {save_battle}>Save Result</button>
        <h1 style = "opacity: {show_battle_over}">Battle not over</h1>
    </div>
</div>

{#if show_over}
    <div class = "show_over">Battle Over!!</div>
{/if}

<style>
    .row {
        display: flex;
    }
    
    .column1_noselect {
        flex: 20%;
        background-color: grey;
        user-select: none;
    }

    .column3 {
        flex: 5;
        background-color: grey;
        display:grid;
    }

    .column2 {
        flex : 75%;
        background-color: aliceblue;
    }
    .hex-row {
        clear: left;
    }
    h1 {
        color: black;
        padding-bottom: 0px;
        margin-bottom: 0px;
        font-size: 20px;
        line-height: 10px;
    }
    .show_over {
        position: absolute; 
        top: 50%; 
        left: 50%;
        background-color: grey;
        font-size: 60px;
    }
</style>

