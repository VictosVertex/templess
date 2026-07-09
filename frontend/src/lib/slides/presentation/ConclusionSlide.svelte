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
			<h3 class="text-4xl font-medium tracking-wide text-[#030c3e] uppercase">Takeaways</h3>
			<p class="mt-4 text-2xl font-bold text-[#8F92AC]">What TempLess improves</p>
		</div>

		<ul class="space-y-10 pl-12">
        <li class="flex flex-col space-y-2">
				<span class="text-3xl font-medium text-[#030c3e]">Workflow is unified</span>
				<span class="text-2xl text-[#8F92AC]">Planning, optimization, and checkout live in the same place.</span>
			</li>
            <li class="flex flex-col space-y-2">
				<span class="text-3xl font-medium text-[#030c3e]">User intent stays central</span>
				<span class="text-2xl text-[#8F92AC]">Templates start from preferences and constraints instead of forcing trial-and-error.</span>
			</li>
			<li class="flex flex-col space-y-2">
				<span class="text-3xl font-medium text-[#030c3e]">Search becomes manageable</span>
				<span class="text-2xl text-[#8F92AC]">Optimization reduces a large manual search space to a guided workflow.</span>
			</li>
		</ul>
	</div>

	<div class="flex flex-col justify-start space-y-12 pl-4 transition-opacity duration-300" class:opacity-0={step < 1}>
		<div class="border-l-4 border-[#9DA322] pl-8">
			<h3 class="text-4xl font-medium tracking-wide text-[#030c3e] uppercase">Next Steps</h3>
			<p class="mt-4 text-2xl font-bold text-[#8F92AC]">Where this can go next</p>
		</div>

		<ul class="space-y-10 pl-6">
			<li class="flex items-center space-x-8">
				<span class="text-4xl font-bold text-[#9DA322]/50">01</span>
				<span class="text-3xl font-medium text-[#030c3e]">Improvements: More presets, optimization, visualization,  ... </span>
			</li>
			<li class="flex items-center space-x-8">
				<span class="text-4xl font-bold text-[#9DA322]/50">02</span>
				<span class="text-3xl font-medium text-[#030c3e]">Features: Explanation, sharing, RoG,...</span>
			</li>
			<li class="flex items-center space-x-8">
				<span class="text-4xl font-bold text-[#9DA322]">03</span>
				<span class="text-3xl font-medium text-[#030c3e]">Provide the tool to users.</span>
			</li>
		</ul>
	</div>
</div>