// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
import React from "react";
import Breadcrumb from "src/layouts/full/shared/breadcrumb/Breadcrumb";
import PageContainer from "src/components/container/PageContainer";
import EventListing from "src/components/apps/events/EventListing";
import { getTranslations } from "./server";
import { useSelector } from "src/store/Store";
import { Box, Button, Grid, Typography } from "@mui/material";

const Blog = async () => { // For instance a Next.js page
  const t = await getTranslations("/page2");
  const t = await getTranslations("/page3");
  const client = useSelector((state) => state.client.client);

  return (
    <PageContainer title={t("My Events")!} description={t("My Events")!}>
      <Breadcrumb
        title={t("My Events")!}
        subtitle={t("Manage your events from here.")!}
        image={client.logo_url}
      />

      <Grid container spacing={3} justifyContent="center" mt={3} mb={3}>
        <Grid item xs={12} sm={10} lg={8} textAlign="center">
          <Typography variant="h2">
            {t("Here are all your events.")}
          </Typography>
          <Typography variant="body1" mt={2}>
            {t("You can manage your events from here. You can create, edit, delete and view your events. Click on \"Add Event\" to create a new event.")}
          </Typography>
        </Grid>
      </Grid>

      <Grid container justifyContent="right" mt={3} mb={3}>
        <Grid item>
          <Button variant="contained" color="primary">
            {t("Add Event")}
          </Button>
        </Grid>
      </Grid>

      <EventListing />
    </PageContainer>
  );
};

export default Blog;
