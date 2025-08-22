package com.epitech.cryptoviz_backend.configuration.database;

import liquibase.Liquibase;
import liquibase.ext.mongodb.database.MongoLiquibaseDatabase;
import liquibase.integration.spring.SpringResourceAccessor;
import lombok.RequiredArgsConstructor;
import lombok.Setter;
import org.springframework.boot.CommandLineRunner;
import org.springframework.context.ResourceLoaderAware;
import org.springframework.core.io.ResourceLoader;
import org.springframework.stereotype.Component;

@Component
@Setter
@RequiredArgsConstructor
public class MongoLiquibaseRunner implements CommandLineRunner, ResourceLoaderAware {

    public final MongoLiquibaseDatabase database;

    protected ResourceLoader resourceLoader;

    @Override
    public void run(String... args) throws Exception {
        Liquibase liquibase = new Liquibase("liquibase/changelog-master.json", new SpringResourceAccessor(resourceLoader), database);
        liquibase.update("dev");
    }
}
