import Card from "./Card.astro";
import CardAction from "./CardAction.astro";
import CardContent from "./CardContent.astro";
import CardDescription from "./CardDescription.astro";
import CardFooter from "./CardFooter.astro";
import CardHeader from "./CardHeader.astro";
import CardTitle from "./CardTitle.astro";

export { Card, CardAction, CardContent, CardDescription, CardFooter, CardHeader, CardTitle };

export default {
  Root: Card,
  Header: CardHeader,
  Footer: CardFooter,
  Title: CardTitle,
  Description: CardDescription,
  Content: CardContent,
  Action: CardAction,
};
