<script lang="ts">
  import SideNav from '../organisms/SideNav.svelte'
</script>

<div class="template">
  <div class="side-nav">
    <SideNav />
  </div>

  <div class="template-slot">
    <slot />
  </div>
</div>

<style>
  .template {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    overflow: hidden;

    display: flex;
    flex-direction: row;
  }
</style>
