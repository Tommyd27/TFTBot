<script>
    //@ts-nocheck

    //export champion to parent component to set
    export let champion;
    let color = "aquamarine"

    //if champion exists and team is 1
    $: if (champion && champion.team) {
        color = "#ffa895"
    }
    else {
        color = "#49ff2d"
    }
</script>
<div class = "hex">
    
    <div class = "top"></div>

    <!--create a button with background color : color, changes dependant on team
        set button to listen to on:click and on:mouseup, parent componenent can define behaviour

        if there is a champion, show the champion id
    -->
    <button on:click on:mouseup style = "background-color: {color}">
        {#if champion}
            {champion.placed_id}
        {/if}
    </button>   
    
    <div class = "bottom"></div>
    
</div>


<!--
    hexagon shape css taken from:
    https://stackoverflow.com/questions/17896791/hexagon-shape-with-css3 
-->
<style>
    .hex {
        float: left;
        margin-left: 3px;
        margin-bottom: -26px; /* sets float to left, sets margin*/  
    }
    .top {
        width: 0;
        border-bottom: 30px solid #6C6;
        border-left: 52px solid transparent;
        border-right: 52px solid transparent;
        z-index: 2;
    }
    button {
        width: 104px;
        height: 60px;
        background: #6C6;
        z-index: 2;
    }
    .bottom {
        width: 0;
        border-top: 30px solid #6C6;
        border-left: 52px solid transparent;
        border-right: 52px solid transparent;
        z-index: 2;
    }
</style>
