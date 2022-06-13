<!-- DOESN'T WORK FOR NOW, PLEASE IGNORE -->

<template>
  <v-card>
    <v-card-title>Add selected text for encryption</v-card-title>
    <v-card-text>{{ selectedText }}</v-card-text>
    <v-card-actions
      ><v-btn color="blue darken-1" @click="addForEncryption(selectedText)"
        >Add</v-btn
      ></v-card-actions
    >
  </v-card>
</template>

<script>
export default {
  // props: ["selectedText", "textSelectedForEncryption"],
  data: () => ({
    return: {},
  }),
  methods: {
    addForEncryption(selectedText) {
      console.log(selectedText);
    },
  },
};
</script>
