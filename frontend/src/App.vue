<template>
  <v-app>
    <v-main>
      <DocumentComponent id="documentComponent" />
    </v-main>
  </v-app>
</template>

<script>
import DocumentComponent from "./components/DocumentComponent.vue";

export default {
  name: "App",

  components: {
    DocumentComponent,
  },
};
</script>
