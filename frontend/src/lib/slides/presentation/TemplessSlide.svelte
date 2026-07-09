<script lang="ts">
  let { isActive } = $props<{ isActive: boolean }>();
  
  let step = $state(0);

  function handleKeydown(e: KeyboardEvent) {
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
  
  <div class="flex flex-col justify-start space-y-12 border-r border-gray-200 pr-12">
    
    <div class="border-l-4 border-[#030c3e] pl-8">
      <h3 class="text-4xl font-medium tracking-wide text-[#5c6079] uppercase">User-centric solution</h3>
      <p class="mt-4 text-2xl font-bold text-[#8F92AC]">Unified experience</p>
    </div>
    
    <ul class="space-y-10 pl-12">
      <li class="flex flex-col space-y-2">
        <span class="text-3xl font-medium text-[#030c3e]">Single Application</span>
        <span class="text-2xl text-[#8F92AC]">A single environment that provides everything.</span>
      </li>
      <li class="flex flex-col space-y-2">
        <span class="text-3xl font-medium text-[#030c3e]">Integrated Item Database</span>
        <span class="text-2xl text-[#8F92AC]">No context switching, integrated persistence.</span>
      </li>
      <li class="flex flex-col space-y-2">
        <span class="text-3xl font-medium text-[#030c3e]">Immediate Feedback</span>
        <span class="text-2xl text-[#8F92AC]">No endless loading bars without feedback.</span>
      </li>
      <li class="flex flex-col space-y-2">
        <span class="text-3xl font-medium text-[#030c3e]">Easy Workflow</span>
        <span class="text-2xl text-[#8F92AC]">Straight forward from creation to finalization.</span>
      </li>
    </ul>

  </div>

  <!-- RIGHT COLUMN: The Optimized Workflow (Revealed on Step 1) -->
  <div class="flex flex-col justify-start space-y-12 pl-4 transition-opacity duration-300" class:opacity-0={step < 1}>
    
    <div class="border-l-4 border-[#9DA322] pl-8">
      <h3 class="text-4xl font-medium tracking-wide text-[#030c3e] uppercase">The Workflow</h3>
      <p class="mt-4 text-2xl font-bold text-[#8F92AC]">From build to checkout in a few steps</p>
    </div>

    <ul class="space-y-10 pl-6">
      <li class="flex items-center space-x-8">
        <span class="text-4xl font-bold text-[#9DA322]/50">01</span>
        <span class="text-3xl font-medium text-[#030c3e]">Create template with sensible defaults.</span>
      </li>
      <li class="flex items-center space-x-8">
        <span class="text-4xl font-bold text-[#9DA322]/50">02</span>
        <span class="text-3xl font-medium text-[#030c3e]">Select must-have items.</span>
      </li>
      <li class="flex items-center space-x-8">
        <span class="text-4xl font-bold text-[#9DA322]/50">03</span>
        <span class="text-3xl font-medium text-[#030c3e]">Tweak preferences (if desired).</span>
      </li>
      <li class="flex items-center space-x-8">
        <span class="text-4xl font-bold text-[#9DA322]/50">04</span>
        <span class="text-3xl font-medium text-[#030c3e]">Click Optimize.</span>
      </li>
      <li class="flex items-center space-x-8">
        <span class="text-4xl font-bold text-[#9DA322]">05</span>
        <span class="text-3xl font-medium text-[#030c3e]">Checkout & review build.</span>
      </li>
    </ul>

  </div>

</div>