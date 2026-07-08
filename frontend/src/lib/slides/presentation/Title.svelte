<script>
  import potasscoTechnologiesLogo from '$lib/assets/potassco_technologies.svg';

  export let title = "TempLess (UI): A User-Centric Interface Approach";
  export let authors = "Workshop on UI/UX for ASP 2026";
  export let affiliation = "Dominik Gerndt - Potassco Solutions GmbH";
  
  // Footer props
  export let footerAuthor = "Dominik Gerndt";
  export let footerTitle = "";
  export let slideNumber = "1 / 15";
</script>
<div class="flex h-dvh w-screen items-center justify-center overflow-hidden bg-[#050505]">
<div class="@container relative aspect-video max-h-dvh w-full max-w-[calc(100dvh*16/9)] flex flex-col overflow-hidden bg-white font-sans text-[#030c3e]">
  
  <main class="flex-1 flex flex-col items-center justify-center px-16 space-y-20">
    
    <h1 class="text-4xl md:text-7xl leading-tight font-medium text-center">
      {title}
    </h1>

    <div class="flex flex-col items-center space-y-5">
      <h2 class="text-2xl md:text-5xl">
        {authors}
      </h2>
      <h3 class="text-3xl text-[#8F92AC]">
        {affiliation}
      </h3>
    </div>

    <div class="pt-20">
      <img
        src={potasscoTechnologiesLogo}
        alt="Potassco Technologies logo"
        class="h-80 w-auto object-contain"
      />
    </div>
    
  </main>

  <footer class="flex justify-between items-center w-full px-4 py-2 text-3xl bg-gray-100 text-black">
    <div class="flex-1 text-left whitespace-nowrap overflow-hidden text-ellipsis">
      {footerAuthor}
    </div>
    
    <div class="flex-1 text-center whitespace-nowrap overflow-hidden text-ellipsis">
      {footerTitle}
    </div>
    
    <div class="flex-1 text-right whitespace-nowrap">
      {slideNumber}
    </div>
  </footer>
  
</div>
</div>