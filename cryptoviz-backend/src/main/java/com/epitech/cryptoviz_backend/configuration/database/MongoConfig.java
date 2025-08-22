package com.epitech.cryptoviz_backend.configuration.database;

import com.mongodb.client.MongoClient;
import com.mongodb.client.MongoClients;
import liquibase.database.DatabaseConnection;
import liquibase.database.DatabaseFactory;
import liquibase.exception.DatabaseException;
import liquibase.ext.mongodb.database.MongoLiquibaseDatabase;
import lombok.AllArgsConstructor;
import lombok.NonNull;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.data.mongodb.config.AbstractMongoClientConfiguration;

import java.util.Optional;

@Configuration
@AllArgsConstructor
public class MongoConfig extends AbstractMongoClientConfiguration {

    @Bean
    @NonNull
    public MongoClient mongoClient() {
        String mongoUri = Optional.ofNullable(System.getenv("MONGO_URI"))
                .orElse("mongodb://root:pass@localhost:27017/cryptoviz?authSource=admin");

        return MongoClients.create(mongoUri);
    }

    @Bean
    @NonNull
    protected String getDatabaseName() {
        Optional<String> databaseName = Optional.ofNullable(System.getenv("DATABASE_NAME"));
        return databaseName.orElse("cryptoviz");
    }

    @Bean
    public MongoLiquibaseDatabase mongoLiquibaseDatabase() throws DatabaseException {
        String mongoUri = Optional.ofNullable(System.getenv("MONGO_URI"))
                .orElse("mongodb://root:pass@localhost:27017/cryptoviz?authSource=admin");

        DatabaseConnection connection = DatabaseFactory.getInstance()
                .openConnection(mongoUri, null, null, null, null);

        MongoLiquibaseDatabase database = new MongoLiquibaseDatabase();
        database.setConnection(connection);
        return database;
    }

    @Bean
    public MongoLiquibaseRunner mongoLiquibaseRunner(MongoLiquibaseDatabase database) {
        return new MongoLiquibaseRunner(database);
    }

}
