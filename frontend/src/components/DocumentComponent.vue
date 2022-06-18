<template fluid>
  <div> 
    <v-btn
      :loading="isSelecting"
      :disabled="loading3"
      color="deep-purple lighten-2"
      class="ma-2 white--text"
      @click="handleFileImport"
    >
      Upload Text
      <v-icon
        right
        dark
      >
        mdi-cloud-upload
      </v-icon>
      </v-btn>
      <input 
        ref="uploader" 
        class="d-none" 
        type="file" 
        @change="onFileChanged"
    >

  <div>
    <v-alert type="success" v-model="alertFinishedEncryption">
      Document encrypted
    </v-alert>
    <v-dialog v-model="dialogSelectionWindowMenu" max-width="500">
      <v-card>
        <v-card-title>Add selected text for encryption</v-card-title>
        <v-select
          :items="roles"
          label="roles"
          single-line
          @change="updateSelectedRole"
        ></v-select>
        <v-card-text>{{ currentFragmentDraft?.content }}</v-card-text>
        <v-card-actions>
          <v-btn color="red" @click="closeSelectionWindowMenu">Cancel</v-btn>
          <v-btn color="blue darken-1" @click="addForEncryption">Add</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <v-card elevation="4" class="mx-auto my-12" max-width="574">
      <v-card-title>This is Lorem Ipsum Document</v-card-title>
      <v-card-text>
        <textarea
          name=""
          id=""
          cols="100"
          rows="10"
          @select="onSelectedText"
          readonly
          v-model="textExample"
        >
        </textarea>
      </v-card-text>
      <v-card-actions>
        <v-btn color="deep-purple lighten-2" @click="finishEncryption">
          Finish Encrypting
        </v-btn>
      </v-card-actions>
    </v-card>
  </div>
</template>

<script>
//import SelectionWindowMenu from "../dialogs/SelectionWindowMenu.vue";
import Document from "../types/document";
import { TechnoKing, Manager, Monkey } from "../types/role";
import TextFragment from "../types/textFragment";
import rolesMap from "../types/textFragment";

let textExample = "";

export default {
  // components: { SelectionWindowMenu },
  data: () => ({
    alertFinishedEncryption: false,
    dialogSelectionWindowMenu: false,
    selectedText: null,
    textSelectedForEncryption: "",
    currentFragmentDraft: null,
    roles: [TechnoKing.name, Manager.name, Monkey.name],
    textExample,
    currentDocument: new Document(textExample),
    selectedRoleName: null,
  }),
  methods: {
    onSelectedText(event) {
      let textArea = event.target;
      let selectedText = textArea.value.slice(
        textArea.selectionStart,
        textArea.selectionEnd
      );

      this.currentFragmentDraft = new TextFragment(
        selectedText,
        textArea.selectionStart,
        textArea.selectionEnd
      );
      this.displaySelectionWindowMenu();
    },
    displaySelectionWindowMenu() {
      this.dialogSelectionWindowMenu = true;
    },
    closeSelectionWindowMenu() {
      this.dialogSelectionWindowMenu = false;
    },
    updateSelectedRole(roleName) {
      this.selectedRoleName = roleName;
      console.log(roleName);
    },
    addForEncryption() {
      let roleName = this.selectedRoleName;
      this.currentFragmentDraft.role = rolesMap[roleName];
      this.currentDocument.addFragment(this.currentFragmentDraft);
      this.currentFragmentDraft = null;
      console.log(roleName);
      this.closeSelectionWindowMenu();
    },
    finishEncryption() {
      this.alertFinishedEncryption = true;
    },
    handleFileImport() {
      // Trigger click on the FileInput
      this.$refs.uploader.click();
    },
    onFileChanged(e) {
      this.selectedFile = e.target.files[0];
          this.readFileContent(this.selectedFile).then(content => {
          this.textExample = content;
          console.log(content);
          console.log(this.textExample);
      }).catch(error => console.log(error));
    },
    readFileContent(file) {
    const reader = new FileReader()
    return new Promise((resolve, reject) => {
      reader.onload = event => resolve(event.target.result);
      reader.onerror = error => reject(error);
      reader.readAsText(file);
    });
    },
  },
};
</script>
<style>
textarea {
  border: none;
  outline: none;
}
</style>
