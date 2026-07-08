<script lang="ts">
  // Accept an isActive prop so the slide knows when it's on screen
  let { isActive } = $props<{ isActive: boolean }>();
  
  let step = $state(0);

  function handleKeydown(e: KeyboardEvent) {
    // If the slide is not active, ignore the keystroke entirely
    if (!isActive) return;

    if (e.code === 'Space') {
      e.preventDefault();
      
      if (step < 2) {
        step++;
      }
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="grid h-full w-full grid-cols-2 gap-24 py-16 pt-40">
  
  <div class="flex flex-col justify-start space-y-12 border-r border-gray-200 pr-12 transition-opacity duration-300" class:opacity-0={step < 1}>
    <div class="border-l-4 border-[#030c3e] pl-8">
      <h3 class="text-4xl font-medium tracking-wide text-[#030c3e] uppercase">Dropped Gear</h3>
      <p class="mt-4 text-2xl font-bold text-[#8F92AC]">Dropped or bought</p>
    </div>
    
    <ul class="space-y-8 pl-12">
      <li class="flex flex-col space-y-2">
        <span class="text-3xl font-medium text-[#030c3e]">1. Uniques</span>
        <span class="text-2xl text-[#8F92AC]">Pre-designed with fixed attributes</span>
      </li>
      <li class="flex flex-col space-y-2">
        <span class="text-3xl font-medium text-[#030c3e]">2. Random World Drops</span>
        <span class="text-2xl text-[#8F92AC]">Randomly generated attributes</span>
      </li>
    </ul>
  </div>

  <div class="flex flex-col justify-start space-y-12 pl-4 transition-opacity duration-300" class:opacity-0={step < 2}>
    <div class="border-l-4 border-[#030c3e] pl-8">
      <h3 class="text-4xl font-medium tracking-wide text-[#030c3e] uppercase">Crafted Gear</h3>
      <p class="mt-4 text-2xl font-bold text-[#8F92AC]">Player-made</p>
    </div>

    <ul class="space-y-8 pl-12">
      <li class="flex flex-col space-y-2">
        <span class="text-3xl font-medium text-[#030c3e]">Gem Slots</span>
        <span class="text-2xl text-[#8F92AC]">4 slots to be filled with stat gems</span>
      </li>
      <li class="flex flex-col space-y-2">
        <span class="text-3xl font-medium text-[#030c3e]">Fifth Bonus</span>
        <span class="text-2xl text-[#8F92AC]">Pre-designed fixed attribute as an additional bonus</span>
      </li>
    </ul>
  </div>

</div>