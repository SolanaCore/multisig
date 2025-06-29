import { createCodamaConfig } from "gill";
 
export default createCodamaConfig({
  idl: "program/idl.json",
  clientJs: "../clients/js/src/generated",
});