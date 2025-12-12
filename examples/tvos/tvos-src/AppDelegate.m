//
//  AppDelegate.m
//  coreaudio-tvos-example
//

#import "AppDelegate.h"
#import "ViewController.h"
@import AVFoundation;

void rust_tvos_main(void);


@interface AppDelegate ()

@end

@implementation AppDelegate



- (BOOL)application:(UIApplication *)application didFinishLaunchingWithOptions:(NSDictionary *)launchOptions {
    // Create window programmatically
    self.window = [[UIWindow alloc] initWithFrame:[[UIScreen mainScreen] bounds]];
    self.window.rootViewController = [[ViewController alloc] init];
    [self.window makeKeyAndVisible];

    NSError *error;
    BOOL success;

    // It is necessary to access the sharedInstance so that calls to AudioSessionGetProperty
    // will work.
    AVAudioSession *session = AVAudioSession.sharedInstance;
    // Setting up the category is not necessary, but generally advised.
    // For tvOS, we use Playback category (PlayAndRecord with DefaultToSpeaker is not available).
    success = [session setCategory:AVAudioSessionCategoryPlayback error:&error];

    if (success) {
        NSLog(@"Calling rust_tvos_main()");
        rust_tvos_main();
    } else {
        NSLog(@"Failed to configure audio session category: %@", error);
    }

    return YES;
}

@end
