export {};
import { ComponentCustomProperties } from "vue";
import { Toasted } from "vue-toasted";

declare module "vue" {
  interface ComponentCustomProperties {
    $toasted: Toasted;
  }
}
