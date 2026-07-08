<script>
  import { LucideCheck, LucideX } from "lucide-svelte";
</script>

<div class="flex flex-col h-full pl-16 md:pl-32 pb-20 text-3xl md:text-5xl gap-16 md:gap-24">
  
  <div class="space-y-4 mt-28">
    <p class="text-[#030c3e] font-semibold pb-8">
      Ignoring the User
    </p>
    <ul class="space-y-3 pl-12">
     <li class="flex items-start gap-4">
        <LucideX class="text-red-500 mt-2 shrink-0" size={40}/>
        <span>"Works on my machine!"</span>
      </li>
      <li class="flex items-start gap-4">
        <LucideX class="text-red-500 mt-2 shrink-0" size={40}/>
        <span>User as the API / Middleman</span>
      </li>
      <li class="flex items-start gap-4">
        <LucideX class="text-red-500 mt-2 shrink-0" size={40}/>
        <span>Only design the happy path</span>
      </li>
    </ul>
  </div>

  <div class="space-y-4">
    <p class="text-[#030c3e] font-semibold pb-8">
      User-centric
    </p>
    <ul class="space-y-3 pl-12">
    <li class="flex items-start gap-4">
        <LucideCheck class="text-[#9DA322] mt-2 shrink-0" size={40}/>
        <span>Built for their reality</span>
      </li>
      <li class="flex items-start gap-4">
        <LucideCheck class="text-[#9DA322] mt-2 shrink-0" size={40}/>
        <span>Clear Workflow with sensible defaults</span>
      </li>
      <li class="flex items-start gap-4">
        <LucideCheck class="text-[#9DA322] mt-2 shrink-0" size={40}/>
        <span>Built for failure</span>
      </li>
    </ul>
  </div>
</div>