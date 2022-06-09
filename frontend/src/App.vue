<template>
  <v-app>
    <v-main>
      <Document />
    </v-main>
  </v-app>
</template>

<script>
import Document from "./components/Document.vue";

export default {
  name: "App",

  components: {
    Document,
  },

  data: () => ({
    //
  }),
};
</script>
