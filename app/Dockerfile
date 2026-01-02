# Stage 1: Build the Flutter web app
FROM dart:stable AS builder

# Install necessary tools
RUN apt-get update && apt-get install -y curl git unzip xz-utils

# Install Flutter SDK by cloning the stable branch
ENV FLUTTER_HOME=/opt/flutter
RUN git clone https://github.com/flutter/flutter.git --depth 1 --branch stable ${FLUTTER_HOME}
ENV PATH="$FLUTTER_HOME/bin:$PATH"

# Run flutter doctor to download any missing components and verify installation
# This will also address the 'dubious ownership' warning by running as root first.
RUN flutter doctor

# Create a non-root user for the build
RUN useradd -ms /bin/bash appuser
# Give the user ownership of the SDK and create a working directory
RUN chown -R appuser:appuser ${FLUTTER_HOME}
RUN mkdir /app && chown -R appuser:appuser /app

# Switch to the non-root user
USER appuser
WORKDIR /app

# Configure Flutter tool for the user
RUN flutter config --no-analytics

# Copy project files
COPY --chown=appuser:appuser pubspec.yaml pubspec.lock ./

# Get dependencies
RUN flutter pub get

# Copy the rest of the source code
COPY --chown=appuser:appuser . .

# Pass build arguments
ARG BASE_URL
ARG WS_URL

# Build the web application
RUN flutter build web --release --dart-define=BASE_URL=${BASE_URL} --dart-define=WS_URL=${WS_URL}

# Stage 2: Create the final production image with Nginx
FROM nginx:alpine

# Install necessary tools if needed (alpine normally has sed)
# RUN apk add --no-cache gettext

# Copy the built web app from the builder stage
COPY --from=builder /app/build/web /usr/share/nginx/html

# Copy the custom Nginx configuration
COPY nginx/nginx.conf /etc/nginx/conf.d/default.conf

# Copy entrypoint script
COPY entrypoint.sh /docker-entrypoint.d/99-config-subst.sh
RUN chmod +x /docker-entrypoint.d/99-config-subst.sh

# Expose port 80
EXPOSE 80

# Command to run Nginx
CMD ["nginx", "-g", "daemon off;"]
