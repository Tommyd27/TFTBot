<script>
    // @ts-nocheck 
    //component imports
    import UnitItemDragDrop from "$lib/UnitItemDragDrop.svelte";
    import Hexagon from "$lib/Hexagon.svelte";
    import HexagonIndent from "$lib/HexagonIndent.svelte";
    import { invoke } from "@tauri-apps/api/tauri"

    //all ids
    let ids = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z']
    //set default values for time unit and time till draw
    let time_unit = 10
    let time_till_draw = 10000
    

    //fetch champs and items func, invoke ipc method
    async function fetch_champs() {
        champs_list = await invoke("retrieve_all_units")
        selected_champ_or_item = champs_list[0]
    }
    async function fetch_items() {
        items_list = await invoke("retrieve_all_items")
    }


    function increase_star_level() {
        grid[show_pos[0]][show_pos[1]].star_level += 1 //increase star level
        if (grid[show_pos[0]][show_pos[1]].star_level > 3) { //make sure it loops around after 3
            grid[show_pos[0]][show_pos[1]].star_level = 1
        }
        update_show(show_pos[0], show_pos[1]) //call update show
    }

    function change_team() {
        grid[show_pos[0]][show_pos[1]].team = !grid[show_pos[0]][show_pos[1]].team //swap team
    }

    function hex_click(i, j) {
        champ = grid[i][j] //set champ to clicked hexagon
        show_pos = [i, j] //set show pos to position
        if (champ == show) { //if already shown, hide
            show = null
        }
        else {
            show = champ //update show
            pos = [80 + 130 * j, 200 + 80 * i]; //change pos depending on position in grid
        }
    }

    //creates empty grid, 2d array 8x8
    function create_grid() {
        let grid = new Array(8)
        for(var i = 0; i < grid.length; i++) {
            grid[i] = new Array(8)
        }
        return grid
    }

    function on_mouse_down(champ, e) {
        //on mouse down of drag and drop
        
        selected_champ_or_item = structuredClone(champ) //clone champ so as not to change by reference the original
        id = champ.id //update id
        show_drag = 100 //show the drag object
        left = e.clientX + 5 //set position to mouse position + 5
        top = e.clientY + 5
    }
    //on mouse up on cell
    function on_mouse_up(i, j) {
        if (show_drag == 100) { //hide drag
            show_drag = 0
            if (selected_champ_or_item.attack_speed_modifier) { //check if has attack_speed_modifier field, if soit is an item
                if (grid[i][j]) { //if there is a champion in that position
                    if (grid[i][j].items[0] == 0) { //update items array to first empty space, or 1st slot
                        grid[i][j].items[0] = selected_champ_or_item.id
                    }
                    else if (grid[i][j].items[1] == 0) {
                        grid[i][j].items[1] = selected_champ_or_item.id
                    }
                    else if (grid[i][j].items[2] == 0) {
                        grid[i][j].items[2] = selected_champ_or_item.id
                    }
                    else {
                        grid[i][j].items[0] = selected_champ_or_item.id
                    }
                }
                update_show(i, j) //update the show, so shows a new item added
            }
            else {
                selected_champ_or_item.items = new Array(0, 0, 0) //set items to empty array
                selected_champ_or_item.star_level = 1 //set star level to 1
                selected_champ_or_item.placed_id = generate_id() //generate new id
                selected_champ_or_item.team = false; //set team to team 1
                grid[i][j] = selected_champ_or_item //update grid
            }
        }
        
        
    }

    function on_mouse_move(e) {
        if (show_drag == 100) { //if dragging something
            left += e.movementX; //add mouse movement
			top += e.movementY;
		}
    }

    function delete_click() { //delete champ at grid location
        grid[show_pos[0]][show_pos[1]] = null; //set both grid and show to null
        show = null;
    }

    function update_show(i, j) {
        //if the cell that has been updated is the same as the cell currently being shown
        if (show_pos[0] == i && show_pos[1] == j && show) {
            show = grid[i][j] //update the show variable
        }
    }

    function log_base_n(base, number) {
        return Math.floor(Math.log(number) / Math.log(base));
    }

    function generate_id() {
        num_placed_champs += 1 //add a new placed champ
        if(num_placed_champs == 0) { //can't log 0, so return A
            return "A"
        }
        let num_to_id = num_placed_champs 
        let id_str = "" //empty string
        let cap = log_base_n(ids.length, num_to_id) //get number of cells
        for(let i = 0; i < cap + 1; i++) {
            if(num_to_id == 0){
                id_str += "A" //cant log 0, so get add A
                break
            }
            let log_n = log_base_n(ids.length, num_to_id) //get log of num_to_id by ids.length rounded down
            let divisor = Math.pow(ids.length, log_n) //calculate divisor
            let id_index = Math.floor(num_to_id / divisor) //get index
            id_str += ids[id_index] //add char to string
            num_to_id -= divisor * id_index //remove num from num to id
        }
        return id_str
    }
    let show_pos = [0, 0] //set the pos being shown to 0, 0
    let id = 0 //set id to 0
    let grid = create_grid() //create the grid

    let champs_list = []
    let items_list = [] //initialise champ and item list
    let show; //initialise show
    let num_placed_champs = -1 //set num placed champs to -1
    let show_drag = 0;
    let pos = [0, 0];
    let selected_champ_or_item;
    let left = 0;
    let top = 0;


    fetch_champs() //fetch champs and items
    fetch_items()

    class PlacedChamp { //create placed champ class to send to backend
        constructor(id, items, star_level, location) {
            this.id = id
            this.items = items
            this.star = star_level
            this.location = location
            this.team = null
        }
    }

    class Location { //create location class to send to backend
        constructor(x, y) {
            this.x = x
            this.y = y
        }
    }

    async function handle_submit() { //send board to backend
        let player_one_champs = []
        let player_two_champs = []
        for (let i = 0; i < grid.length; i++) { //for each grid cell, if a champ is placed there, create a placed champ from the information and push to respective team
            for (let j = 0; j < grid[i].length; j++) {
                if (grid[i][j]) {
                    let placed_champ = new PlacedChamp(grid[i][j].id, grid[i][j].items, grid[i][j].star_level, new Location(i, j))
                    if (grid[i][j].team) {
                        player_one_champs.push(placed_champ)
                    }
                    else {
                        player_two_champs.push(placed_champ)
                    }
                }
            }
        } //submit board
        await invoke("submit_board", {"playerOneChamps": player_one_champs, "playerTwoChamps": player_two_champs, "timeUnit": time_unit, "timeTillDraw" : time_till_draw})
        window.location.href = "/board/battle" //move to battle page
    }

    let champ = champs_list[0]
</script>
<svelte:window on:mousemove={on_mouse_move} />
<h1>Board</h1>

<div class="row">
    <div class="column1_noselect"> <!--left column: create drag and drop for each chmap and item-->
        <h2 style = "font-size: 20px">Champs</h2><br>
        {#each champs_list as champ} <!--bound on mouse down to on mouse down function with selected champ-->
            <UnitItemDragDrop champ_or_item = {champ} on:mousedown = {(e) => on_mouse_down(champ, e)}></UnitItemDragDrop>
        {/each}
        <h2 style = "font-size: 20px">Items</h2><br>
        {#each items_list as item}
            <UnitItemDragDrop champ_or_item = {item} on:mousedown = {(e) => on_mouse_down(item, e)}></UnitItemDragDrop>
        {/each}
    </div>
    <div class="column2">
        {#each grid as row, i} <!--create hexagon grid-->
            {#if i % 2 == 1}
                <HexagonIndent></HexagonIndent>
            {/if}
            {#each row as hex, j} <!-- set on click to hex click location, on mouse up to on mouse up with location-->
                <Hexagon champion = {grid[i][j]} on:click = {() => hex_click(i, j)} on:mouseup = {() => on_mouse_up(i, j)}></Hexagon>
            {/each}
            <div class = "hex-row"></div>
        {/each}
    </div>
    <div class="column3">
        <form on:submit={handle_submit}> <!--create a form, with on submit = handle submit-->
            <button type = "submit">Start Battle</button><br>
            <label>Time Unit</label>
            <input type = "number" min = 1 max = 1000 bind:value = {time_unit} required><br> <!-- create an input with min 1 max 1000 and type number, so only numeric inputs allow, also require for submission-->
            <label>Ticks Till Draw</label>
            <input type = "number" min = 100 max = 100000 bind:value = {time_till_draw} required> <!--create an input with similar checks to ensure value is valid-->
        </form>
    </div>
</div>

<div class = "drag_container" style = "position: absolute; left: {left}px; top: {top}px; opacity: {show_drag}"> <!--drag container, what shows when user is dragging a champ or item-->
    <h1>{id}</h1>
</div>

{#if show} <!--if show champ info-->
    <div style = "position: absolute; top: {pos[1]}px; left: {pos[0]}px" class = "info_bar"> <!--show champ info at location-->
        <h1>Champ: {show.placed_id}</h1>
        <h1>Type: {show.id}</h1>
        <h1>Star Level: {show.star_level}</h1>
        <h1>Items: {show.items}</h1>
        <button on:click = {delete_click}>Delete Unit</button>
        <button on:click = {increase_star_level}>Increase Star Level</button>
        <button on:click = {change_team}>Change Team</button>
    </div>
{/if}


<style>
    /* create flex box for 3 column split */
    .row {
        display: flex;
    }
    
    /*split column 1 into 2 columns 
    give right space to each flex column
    */
    .column1_noselect {
        flex: 5%;
        background-color: grey;
        display:grid;
        grid-template-columns: 40px 40px;
        user-select: none; /* make sure user select disable to user doesnt "select"/ highlight a bunch of the drag and drop when moving */
    }

    .column3 {
        flex: 15%;
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

    .info_bar {
        width: 140px;
        height: 160px;
        font-size: 10px;
        background-color: aliceblue;
        z-index: 3;
    }

    .drag_container {
        border: solid 1px gray;
        background-color: aliceblue;
    }
</style>

