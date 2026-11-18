import Popover from "./Popover.astro";
import PopoverContent from "./PopoverContent.astro";
import PopoverDescription from "./PopoverDescription.astro";
import PopoverHeader from "./PopoverHeader.astro";
import PopoverTitle from "./PopoverTitle.astro";
import PopoverTrigger from "./PopoverTrigger.astro";

export {
  Popover,
  PopoverContent,
  PopoverDescription,
  PopoverHeader,
  PopoverTitle,
  PopoverTrigger,
};

export default {
  Root: Popover,
  Trigger: PopoverTrigger,
  Content: PopoverContent,
  Header: PopoverHeader,
  Title: PopoverTitle,
  Description: PopoverDescription,
};
