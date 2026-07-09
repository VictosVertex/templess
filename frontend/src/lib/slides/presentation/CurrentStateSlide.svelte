<script lang="ts">
  // Accept an isActive prop so the slide knows when it's on screen
  let { isActive } = $props<{ isActive: boolean }>();
  
  let step = $state(0);

  function handleKeydown(e: KeyboardEvent) {
    // If the slide is not active, ignore the keystroke entirely
    if (!isActive) return;

    if (e.code === 'Space') {
      e.preventDefault();
      
      if (step < 1) {
        step++;
      }
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="grid h-full w-full grid-cols-2 gap-24 py-16">
  
  <!-- LEFT COLUMN: The Tools (Always visible) -->
  <div class="flex flex-col justify-start space-y-12 border-r border-gray-200 pr-12">
    
    <div class="border-l-4 border-[#030c3e] pl-8">
      <h3 class="text-4xl font-medium tracking-wide text-[#030c3e] uppercase">No Singular Solution</h3>
      <p class="mt-4 text-2xl font-bold text-[#8F92AC]">A Fragmented Ecosystem</p>
    </div>
    
    <ul class="space-y-10 pl-12">
      <li class="flex flex-col space-y-2">
        <span class="text-3xl font-medium text-[#030c3e]">Template Planner</span>
        <span class="text-2xl text-[#8F92AC]">A standalone application used to calculate stats and track the build.</span>
      </li>
      <li class="flex flex-col space-y-2">
        <span class="text-3xl font-medium text-[#030c3e]">Item Database</span>
        <span class="text-2xl text-[#8F92AC]">Server website hosting the entire item database.</span>
      </li>
      <li class="flex flex-col space-y-2">
        <span class="text-3xl font-medium text-[#030c3e]">Attempted Optimizer</span>
        <span class="text-2xl text-[#8F92AC]">Another attempt to solve the gap.</span>
      </li>
    </ul>

  </div>

  <!-- RIGHT COLUMN: The Workflow (Revealed on Step 1) -->
  <div class="flex flex-col justify-start space-y-12 pl-4 transition-opacity duration-300" class:opacity-0={step < 1}>
    
    <div class="border-l-4 border-red-500 pl-8">
      <h3 class="text-4xl font-medium tracking-wide text-[#030c3e] uppercase">The Workflow</h3>
      <p class="mt-4 text-2xl font-bold text-[#8F92AC]">Manual Trial and Error</p>
    </div>

    <ul class="space-y-10 pl-6">
     <li class="flex items-center space-x-8">
        <span class="text-4xl font-bold text-red-500/50">01</span>
        <span class="text-3xl font-medium text-[#030c3e]">Use database and planner.</span>
      </li>
      <li class="flex items-center space-x-8">
        <span class="text-4xl font-bold text-red-500/50">02</span>
        <span class="text-3xl font-medium text-[#030c3e]">Manually search the database for items.</span>
      </li>
      <li class="flex items-center space-x-8">
        <span class="text-4xl font-bold text-red-500/50">03</span>
        <span class="text-3xl font-medium text-[#030c3e]">Click a button to copy the item information.</span>
      </li>
      <li class="flex items-center space-x-8">
        <span class="text-4xl font-bold text-red-500/50">04</span>
        <span class="text-3xl font-medium text-[#030c3e]">Switch to planner and paste into item slot.</span>
      </li>
      <li class="flex items-start space-x-8 pt-4">
        <span class="text-4xl font-bold text-red-500">05</span>
        <div class="flex flex-col space-y-2">
          <span class="text-3xl font-bold text-[#030c3e]">Repeat endlessly.</span>
          <span class="text-2xl text-[#8F92AC]">Manually solve the entire template optimization.</span>
        </div>
      </li>
      <li class="flex items-start space-x-8 pt-4">
        <span class="text-4xl font-bold text-red-500">06</span>
        <div class="flex flex-col space-y-2">
          <span class="text-3xl font-bold text-[#030c3e]">Gather item source information</span>
          <span class="text-2xl text-[#8F92AC]">Manually find item prices and create a checklist.</span>
        </div>
      </li>
    </ul>

  </div>

</div>