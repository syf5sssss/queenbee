import { createApp } from 'vue'
import App from './App.vue'
import PrimeVue from "primevue/config";
import Aura from '@primevue/themes/aura';
import 'primeflex/primeflex.css';
import ToastService from 'primevue/toastservice';


const app = createApp(App);
app.use(ToastService);

app.use(PrimeVue, {
    theme: {
        preset: Aura
    }
});
app.mount('#app');
