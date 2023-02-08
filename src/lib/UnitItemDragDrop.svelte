<script>
    // @ts-nocheck 

    export let champ_or_item;
</script>

<!--listen to mousedown event, export behaviour to parent component, show champ or item id-->
<div on:mousedown>
    <h1>{champ_or_item.id}</h1>
    
</div>

<style>
    /*white on black styling for clear display/ reading. outline for more clarity */
    div {
        outline: 2px dashed blue;
        background-color: white;
        width: 30px;
        height: 30px;
       
    }
    h1 {
        color: black;
        text-align: center;
        margin-top: -4px;
    }
</style>