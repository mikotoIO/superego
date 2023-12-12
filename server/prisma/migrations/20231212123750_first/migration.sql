-- CreateTable
CREATE TABLE "Identity" (
    "id" UUID NOT NULL,
    "username" STRING(64) NOT NULL,
    "userType" STRING(64),
    "displayName" STRING(64) NOT NULL,

    CONSTRAINT "Identity_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Credential" (
    "id" UUID NOT NULL,
    "email" STRING(256) NOT NULL,
    "passhash" STRING(256) NOT NULL,
    "userId" UUID NOT NULL,

    CONSTRAINT "Credential_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Session" (
    "id" UUID NOT NULL,
    "token" STRING(256) NOT NULL,
    "createdAt" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "expiresAt" TIMESTAMP,
    "userId" UUID NOT NULL,
    "serviceKeyId" UUID NOT NULL,

    CONSTRAINT "Session_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Service" (
    "id" UUID NOT NULL,
    "name" STRING(64) NOT NULL,
    "description" STRING(256),
    "domain" STRING(128) NOT NULL,
    "ownerID" UUID,

    CONSTRAINT "Service_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "ServiceKey" (
    "id" UUID NOT NULL,
    "serviceId" UUID NOT NULL,
    "algorithm" STRING(64) NOT NULL,
    "domain" STRING(128) NOT NULL,
    "signingKey" STRING(1024) NOT NULL,
    "publicKey" STRING(1024),

    CONSTRAINT "ServiceKey_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "Identity_username_key" ON "Identity"("username");

-- CreateIndex
CREATE UNIQUE INDEX "Credential_email_key" ON "Credential"("email");

-- CreateIndex
CREATE UNIQUE INDEX "Session_token_key" ON "Session"("token");

-- CreateIndex
CREATE UNIQUE INDEX "Service_name_key" ON "Service"("name");

-- CreateIndex
CREATE UNIQUE INDEX "Service_domain_key" ON "Service"("domain");

-- CreateIndex
CREATE UNIQUE INDEX "ServiceKey_serviceId_key" ON "ServiceKey"("serviceId");

-- CreateIndex
CREATE UNIQUE INDEX "ServiceKey_domain_key" ON "ServiceKey"("domain");

-- AddForeignKey
ALTER TABLE "Credential" ADD CONSTRAINT "Credential_userId_fkey" FOREIGN KEY ("userId") REFERENCES "Identity"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Session" ADD CONSTRAINT "Session_userId_fkey" FOREIGN KEY ("userId") REFERENCES "Identity"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Session" ADD CONSTRAINT "Session_serviceKeyId_fkey" FOREIGN KEY ("serviceKeyId") REFERENCES "ServiceKey"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Service" ADD CONSTRAINT "Service_ownerID_fkey" FOREIGN KEY ("ownerID") REFERENCES "Identity"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "ServiceKey" ADD CONSTRAINT "ServiceKey_serviceId_fkey" FOREIGN KEY ("serviceId") REFERENCES "Service"("id") ON DELETE CASCADE ON UPDATE CASCADE;
