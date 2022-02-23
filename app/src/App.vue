<script setup>
import { useRoute } from 'vue-router'
import TheSidebar from './components/TheSidebar'
import { PhantomWalletAdapter, SolflareWalletAdapter } from '@solana/wallet-adapter-wallets'
import { initWallet } from 'solana-wallets-vue'
import { initWorkspace } from './composables/useWorkspace'

const route = useRoute();

const wallets = [
  new PhantomWalletAdapter(),
  new SolflareWalletAdapter(),
];

initWallet({ wallets, autoConnect: true });
initWorkspace()

</script>

<template>
    <div class="w-full max-w-3xl mx-auto lg:max-w-4xl">

        <!-- Sidebar. -->
        <the-sidebar class="fixed w-20 py-4 md:py-8 md:pl-4 md:pr-8 md:w-64"></the-sidebar>

        <!-- Main -->
        <main class="flex-1 min-h-screen ml-20 border-l border-r md:ml-64">
            <header class="flex items-center justify-between px-8 py-4 border-b space-x-6">
                <div class="text-xl font-bold" v-text="route.name"></div>
            </header>
            <router-view></router-view>
        </main>
    </div>
</template>
