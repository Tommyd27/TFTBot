<script>
    // @ts-nocheck 
    
    //import hexagon and hexagon indent
    import Hexagon from "$lib/Hexagon.svelte";
    import HexagonIndent from "$lib/HexagonIndent.svelte";
    import { invoke } from "@tauri-apps/api/tauri"

    let play = false; //let currently playing be false

    //alphabet for id list
    let ids = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z']

    //creates an empty grid, 2d array 8x8
    function create_grid() {
        grid = new Array(8) //creates empty 8 by 8 array
        for(var i = 0; i < grid.length; i++) {
            grid[i] = new Array(8) //for i in grid, set i to an array 8 long
        }
    }

    //logs number with base N
    function log_base_n(base, number) {
        return Math.floor(Math.log(number) / Math.log(base));
    }
    //generates an id given a num
    function generate_id(num_to_id) {
        if(num_to_id == 0) { //return A
            return "A"
        }
        if(num_to_id == 255) {
            return "None"
        }
        let id_str = "" //initalise string to empty
        let cap = log_base_n(ids.length, num_to_id) //get number of characters
        for(let i = 0; i < cap + 1; i++) { //for each character
            if(num_to_id == 0){
                id_str += "A" //add A as you cannot logarithm 0
                break
            }
            //get log of num_to_id with ids.length, rounded down
            let log_n = log_base_n(ids.length, num_to_id)
            //get divisor
            let divisor = Math.pow(ids.length, log_n)
            //get id from divisor
            let id_index = Math.floor(num_to_id / divisor)
            id_str += ids[id_index]
            //minus divisor
            num_to_id -= divisor * id_index
        }
        //return new string
        return id_str
    }

    //set the selected champ to the champ at location
    function hex_click(i, j) {
        selected_champ = grid[i][j]
    }
    async function fetch_board() {
        create_grid() //creates a grid
        //fetches the board from backend
        board = await invoke("fetch_board")
        //if one side has no champions, declare winner
        if (board.p1_champions.length == 0 || board.p2_champions.length == 0) {
            show_over = true
            //show winner then hide after 2 seconds
            await new Promise(r => setTimeout(r, 2000));
            show_over = false
        }
        //for champ in p1 champs
        for (let champ_index in board.p1_champions) {
            //get the location
            let location = board.p1_champions[champ_index].location
            board.p1_champions[champ_index].team = true //set the team to true/ team 1
            board.p1_champions[champ_index].placed_id = generate_id(board.p1_champions[champ_index].id) //generate an id for it
            grid[location.x][location.y] = board.p1_champions[champ_index] //set the grid positon at location to the new champ
        }
        //repeat for player 2 champs
        for (let champ_index in board.p2_champions) {
            let location = board.p2_champions[champ_index].location
            board.p2_champions[champ_index].team = false
            board.p2_champions[champ_index].placed_id = generate_id(board.p2_champions[champ_index].id)
            grid[location.x][location.y] = board.p2_champions[champ_index]
        }
    }
    //pauses the playthrough
    function pause() {
        play = false;
    }
    //plays the simulation
    async function play_at() {
        if (play) { return } //if already playing, return
        play = true //set play to true
        while (play) { //while play
            let start = Date.now()
            await invoke("simulate_x_ticks", {numTicks : play_at_ticks})
            await fetch_board() //simulate ticks
            let time_taken = Date.now() - start //calculate time taken to simulate ticks and calculate how long to wait for another second
            if (time_taken < 1000) {
                await new Promise(r => setTimeout(r, 1000 - time_taken)); //wait until a second passes
            } //repeat until paused
        }
        
    }
    async function jump_forward() { //simulate x ticks then fetch board
        if (play) { return }
        await invoke("simulate_x_ticks", {numTicks : jump_ticks_num})
        await fetch_board()
    }
    async function save_battle() {
        if (board.numTicks != board.ticks_till_draw  && (board.p1_champions.length > 0 && board.p2_champions.length > 0) ) { //if battle isn't over
            show_battle_over = 100 //show battle not over text
            await new Promise(r => setTimeout(r, 500));
            show_battle_over = 0
            return //return
        }
        
        let outcome = 0 //default outcome for draw
        if (board.p1_champions.length > 0 && board.p2_champions.length > 0) {
            outcome = 0 //set as draw
        }
        else if (board.p1_champions.length > 0) { 
            outcome = 1// set winner to p1
        }
        else {
            outcome = 2 //set winner to p2
        }
        invoke("update_outcome", {outcome}) //update outcome

    }
    let grid; //initialise empty grid
    let play_at_ticks = 5; //set play at ticks to 5
    let jump_ticks_num = 100; //set jump ticks to 100
    let board; //initialise empty board
    let selected_champ; //initialise empty selected champ
    let show_battle_over = 0 //set show battle over and show over to false
    let show_over = false;
    fetch_board() //fetch board
</script>

<div class="row">
    <div class="column1_noselect"> <!--left column-->
        {#if selected_champ} <!--if selected champ, display all stats on left side-->
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
    <div class="column2"> <!--mid column-->
        {#each grid as row, i} <!--for each row and index in grid, put indent if odd row-->
            {#if i % 2 == 1}
                <HexagonIndent></HexagonIndent>
            {/if}
            {#each row as hex, j} <!-- create hexagon with champ at location i j-->
                
                <Hexagon champion = {grid[i][j]} on:click = {() => hex_click(i, j)}></Hexagon>
            {/each}
            <div class = "hex-row"></div> <!-- create new row-->
        {/each}
    </div>
    <div class="column3"> <!--right column-->
        <button on:click = {play_at}>Play</button> <!--inputs and buttons to control playback-->
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

{#if show_over} <!--show battle not over-->
    <div class = "show_over">Battle Over!!</div>
{/if}

<style>
    .row { /* flex display*/
        display: flex;
    }
    /* set column flexs to correct width */
    .column1_noselect {
        flex: 20%;
        background-color: grey;
        user-select: none;
    }

    .column3 {
        flex: 5%;
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

